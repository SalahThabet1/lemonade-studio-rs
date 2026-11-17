#!/usr/bin/env python3
"""
Lemonade Studio - Neural AI Background Remover (BRIA RMBG-1.4 / RMBG-2.0)
Semantic foreground segmentation for people, characters, and complex objects.
"""

import argparse
import os
import sys
import numpy as np
from PIL import Image, ImageFilter

def find_default_model():
    candidates = [
        os.path.expanduser("~/models/rmbg/rmbg-1.4.onnx"),
        os.path.expanduser("~/models/rmbg/rmbg-2.0.onnx"),
    ]
    for p in candidates:
        if os.path.isfile(p):
            return p
    return candidates[0]

def run_rmbg(input_path, output_path, model_path=None, tolerance=0.5, pixel_mode=False, halo_trim=0, mask_out_path=None):
    try:
        import onnxruntime as ort
    except ImportError:
        print("ERROR: onnxruntime is not installed.", file=sys.stderr)
        sys.exit(1)

    if model_path is None or not os.path.isfile(model_path):
        model_path = find_default_model()

    if not os.path.isfile(model_path):
        print(f"ERROR: Model file not found at: {model_path}", file=sys.stderr)
        sys.exit(2)

    # Load session
    opts = ort.SessionOptions()
    opts.graph_optimization_level = ort.GraphOptimizationLevel.ORT_ENABLE_ALL
    opts.intra_op_num_threads = max(2, min(8, os.cpu_count() or 4))
    session = ort.InferenceSession(model_path, sess_options=opts, providers=["CPUExecutionProvider"])

    img = Image.open(input_path).convert("RGB")
    orig_w, orig_h = img.size

    # Prepare 1024x1024 input tensor for RMBG-1.4
    target_size = (1024, 1024)
    resized = img.resize(target_size, Image.Resampling.BILINEAR)
    im_arr = np.array(resized, dtype=np.float32) / 255.0
    im_arr = (im_arr - 0.5) / 1.0
    tensor = np.transpose(im_arr, (2, 0, 1))[np.newaxis, ...]

    # Run inference
    input_name = session.get_inputs()[0].name
    output_name = session.get_outputs()[0].name
    outputs = session.run([output_name], {input_name: tensor.astype(np.float32)})

    raw_out = np.squeeze(outputs[0])
    min_v = float(np.min(raw_out))
    max_v = float(np.max(raw_out))
    if max_v > min_v:
        mask_out = (raw_out - min_v) / (max_v - min_v)
    else:
        mask_out = np.clip(raw_out, 0.0, 1.0)

    # Resize mask to original resolution
    mask_img = Image.fromarray((mask_out * 255.0).astype(np.uint8))
    if (mask_img.width, mask_img.height) != (orig_w, orig_h):
        mask_img = mask_img.resize((orig_w, orig_h), Image.Resampling.BILINEAR)

    # Save raw neural mask if requested for real-time live interactive tuning
    if mask_out_path:
        os.makedirs(os.path.dirname(os.path.abspath(mask_out_path)), exist_ok=True)
        mask_img.save(mask_out_path)

    mask_arr = np.array(mask_img, dtype=np.float32) / 255.0

    # Apply alpha threshold
    if pixel_mode:
        # Binary 1-bit alpha for retro pixel art
        alpha = np.where(mask_arr >= tolerance, 255, 0).astype(np.uint8)
    else:
        # Smooth anti-aliased gradient around threshold
        low = max(0.0, tolerance - 0.15)
        high = min(1.0, tolerance + 0.15)
        if high > low:
            scaled = np.clip((mask_arr - low) / (high - low), 0.0, 1.0)
        else:
            scaled = np.where(mask_arr >= tolerance, 1.0, 0.0)
        alpha = (scaled * 255.0).astype(np.uint8)

    alpha_img = Image.fromarray(alpha, mode="L")

    # Halo trim / edge erosion using PIL MinFilter
    if halo_trim > 0:
        filter_size = halo_trim * 2 + 1
        alpha_img = alpha_img.filter(ImageFilter.MinFilter(filter_size))

    rgba = img.convert("RGBA")
    r, g, b, _ = rgba.split()
    cutout = Image.merge("RGBA", (r, g, b, alpha_img))

    os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
    cutout.save(output_path)
    print(f"SUCCESS: Cutout saved to {output_path} ({orig_w}x{orig_h})")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Neural AI Background Remover")
    parser.add_argument("input", help="Path to input image")
    parser.add_argument("output", help="Path to output PNG")
    parser.add_argument("--model", default=None, help="Path to RMBG ONNX model")
    parser.add_argument("--tolerance", type=float, default=0.5, help="Alpha threshold (0.0 - 1.0)")
    parser.add_argument("--pixel-mode", action="store_true", help="1-bit binary alpha for pixel art")
    parser.add_argument("--halo-trim", type=int, default=0, help="Erode edge pixels")
    parser.add_argument("--mask-out", default=None, help="Path to save raw grayscale mask PNG")

    args = parser.parse_args()
    run_rmbg(
        input_path=args.input,
        output_path=args.output,
        model_path=args.model,
        tolerance=args.tolerance,
        pixel_mode=args.pixel_mode,
        halo_trim=args.halo_trim,
        mask_out_path=args.mask_out,
    )
