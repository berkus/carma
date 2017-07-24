# Model ../DecodedData/DATA/MODELS/SCREWIE.DAT
# Actor ../DecodedData/DATA/ACTORS/SCREWIE.ACT
# Material ../DecodedData/DATA/MATERIAL/VLADCROM.MAT
# Pixelmap ../DecodedData/DATA/PIXELMAP/CHROME.PIX
meta:
  id: carmageddon_resource
  title: Carmageddon resource file definition
  application: Carmageddon 1 game data files
  endian: be
seq:
  - id: header
    type: chunk
  - id: chunks
    type: chunk
    repeat: eos
types:
  chunk_header:
    seq:
      - id: type
        type: u4
        enum: chunk_type
      - id: size
        type: u4
    instances:
      size:
        value: size
        if: type != chunk_type::material_list
      size:
        value: size + 16
        if: type == chunk_type::material_list
  chunk:
    seq:
      - id: header
        type: chunk_header
      - id: body
        #size: header.size - it's incorrect for several chunks, notably material_list
        type:
          switch-on: header.type
          cases:
            chunk_type::file_header: file_header_chunk
            chunk_type::pixelmap_header: pixelmap_header_chunk
            chunk_type::pixelmap_ref: pixelmap_ref_chunk
            chunk_type::rendertab_ref: rendertab_ref_chunk
            chunk_type::meshfile_ref: meshfile_ref_chunk
            chunk_type::material_ref: material_ref_chunk
            chunk_type::material_list: material_list_chunk
            chunk_type::vertex_list: vertex_list_chunk
            chunk_type::uvmap_list: uvmap_list_chunk
            chunk_type::face_mat_list: face_mat_list_chunk
            chunk_type::material_desc: material_desc_chunk
            chunk_type::pixelmap_data: pixelmap_data_chunk
            chunk_type::actor_transform: actor_transform_chunk
            chunk_type::face_list: face_list_chunk
            chunk_type::actor_name: actor_name_chunk
            chunk_type::file_name: file_name_chunk
            chunk_type::map_boundingbox: map_boundingbox_chunk
  file_header_chunk:
    seq:
      - id: file_type
        type: u4
        enum: file_type
      - id: dummy
        type: u4
  pixelmap_header_chunk:
    seq:
      - id: what1
        type: u1
      - id: width
        type: u2
      - id: use_width
        type: u2
      - id: height
        type: u2
      - id: use_height
        type: u2
      - id: what2
        type: u2
      - id: name
        type: strz
        encoding: ascii
  pixelmap_data_chunk:
    seq:
      - id: units
        type: u4
      - id: unit_bytes
        type: u4
      - id: pixel_data
        size: payload_size
    instances:
      payload_size:
        value: units * unit_bytes
  file_name_chunk:
    seq:
      - id: subtype
        type: u2
        enum: file_subtype
      - id: name
        type: strz
        encoding: ascii
  vertex_list_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: vertices
        type: vec3f
        repeat: expr
        repeat-expr: entries_count
  uvmap_list_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: uvcoords
        type: uvcoordf
        repeat: expr
        repeat-expr: entries_count
  face_list_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: faces
        type: face
        repeat: expr
        repeat-expr: entries_count
  material_list_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: materials
        type: strz
        encoding: ascii
        repeat: expr
        repeat-expr: entries_count
  face_mat_list_chunk:
    seq:
      - id: entries_count
        type: u4
      - id: unknown
        type: u4
      - id: face_materials # actually mat_id+1
        type: u2
        repeat: expr
        repeat-expr: entries_count
  vec3f:
    seq:
      - id: x
        type: f4
      - id: y
        type: f4
      - id: z
        type: f4
  uvcoordf:
    seq:
      - id: u
        type: f4
      - id: v
        type: f4
  face:
    seq:
      - id: v1
        type: u2
      - id: v2
        type: u2
      - id: v3
        type: u2
      - id: flags
        type: u2
      - id: material_index # probably not as we have face_mat_list for that
        type: u1
  actor_name_chunk:
    seq:
      - id: visible
        type: u1
      - id: unknown
        type: u1
      - id: name
        type: strz
        encoding: ascii
  actor_transform_chunk:
    seq:
      - id: scale_row1
        type: vec3f
      - id: scale_row2
        type: vec3f
      - id: scale_row3
        type: vec3f
      - id: translate_row4
        type: vec3f
  meshfile_ref_chunk:
    seq:
      - id: name
        type: strz
        encoding: ascii
  material_ref_chunk:
    seq:
      - id: name
        type: strz
        encoding: ascii
  pixelmap_ref_chunk:
    seq:
      - id: name
        type: strz
        encoding: ascii
  rendertab_ref_chunk:
    seq:
      - id: name
        type: strz
        encoding: ascii
  material_desc_chunk:
    seq:
      - id: params
        type: f4
        repeat: expr
        repeat-expr: 12
      - id: name
        type: strz
        encoding: ascii
  map_boundingbox_chunk:
    seq:
      - id: low
        type: vec3f
      - id: high
        type: vec3f
enums:
  chunk_type:
    #0x0: null
    0x3: pixelmap_header
    0x4: material_desc
    0x12: file_header
    0x16: material_list
    0x17: vertex_list
    0x18: uvmap_list
    0x1a: face_mat_list
    0x1c: pixelmap_ref
    0x1f: rendertab_ref
    0x21: pixelmap_data
    0x23: actor_name
    0x25: unknown_25 # no chunk data
    0x24: meshfile_ref
    0x26: material_ref
    0x29: unknown_29 # no chunk data
    0x2a: unknown_2a # no chunk data
    0x2b: actor_transform
    0x32: map_boundingbox
    0x35: face_list
    0x36: file_name
  file_type:
    0x1: actor
    0x2: pixelmap
    0x5: material
    0xface: mesh
  file_subtype:
    3: model
