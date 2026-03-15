
void __thiscall FUN_0052d920(void *this,void *param_1,undefined4 param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  void *pvVar2;
  undefined3 extraout_var_00;
  undefined4 *puVar3;
  undefined4 local_238 [3];
  undefined2 local_22c;
  undefined2 local_22a;
  undefined2 local_228;
  undefined2 local_226;
  undefined2 local_224;
  undefined2 local_222;
  undefined2 local_220;
  undefined2 local_21e;
  undefined2 local_21c;
  undefined2 local_21a;
  undefined2 local_218;
  undefined2 local_216;
  undefined2 local_214;
  undefined2 local_212;
  undefined2 local_210;
  undefined2 local_20e;
  undefined2 local_20c;
  undefined2 local_20a;
  undefined2 local_208;
  undefined2 local_206;
  undefined2 local_204;
  undefined2 local_202;
  undefined2 local_200;
  undefined2 local_1fe;
  undefined2 local_1fc;
  undefined2 local_1fa;
  undefined2 local_1f8;
  undefined2 local_1f6;
  undefined2 local_1f4;
  undefined2 local_1f2;
  undefined2 local_1f0;
  undefined2 local_1ee;
  undefined2 local_1ec;
  undefined2 local_1ea;
  undefined2 local_1e8;
  undefined2 local_1e6;
  undefined2 local_1e4;
  undefined2 local_1e2;
  undefined2 local_1e0;
  undefined2 local_1de;
  undefined2 local_1dc;
  undefined2 local_1da;
  undefined2 local_1d8;
  undefined2 local_1d6;
  undefined2 local_1d4;
  undefined2 local_1d2;
  undefined2 local_1d0;
  undefined2 local_1ce;
  undefined2 local_1cc;
  undefined2 local_1ca;
  undefined2 local_1c8;
  undefined2 local_1c6;
  undefined2 local_1c4;
  undefined2 local_1c2;
  undefined2 local_1c0;
  undefined2 local_1be;
  undefined2 local_1bc;
  undefined2 local_1ba;
  undefined2 local_1b8;
  undefined2 local_1b6;
  undefined2 local_1b4;
  undefined2 local_1b2;
  undefined2 local_1b0;
  undefined2 local_1ae;
  undefined2 local_1ac;
  undefined2 local_1aa;
  undefined2 local_1a8;
  undefined2 local_1a6;
  undefined4 local_1a4 [3];
  undefined4 local_198 [3];
  undefined4 local_18c [3];
  undefined4 local_180 [3];
  undefined4 local_174 [3];
  undefined4 local_168 [3];
  undefined4 local_15c [3];
  undefined4 local_150 [3];
  undefined4 local_144 [3];
  undefined4 local_138 [3];
  undefined4 local_12c [3];
  undefined4 local_120 [3];
  undefined4 local_114 [3];
  undefined4 local_108 [3];
  undefined4 local_fc [3];
  undefined4 local_f0 [3];
  undefined4 local_e4 [3];
  undefined4 local_d8 [3];
  undefined4 local_cc [3];
  undefined4 local_c0 [3];
  undefined4 local_b4 [3];
  undefined4 local_a8 [3];
  undefined4 local_9c [3];
  undefined4 local_90 [3];
  undefined4 local_84 [3];
  undefined4 local_78 [3];
  undefined4 local_6c [3];
  undefined4 local_60 [3];
  undefined4 local_54 [3];
  undefined4 local_48 [3];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006451e6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_238);
  local_4 = 0;
  switch(param_2) {
  case 1:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_20a = DAT_0065d424;
      local_20c = 0x4d50;
      pvVar2 = FUN_005f2fc0(local_3c,&local_20c);
      local_4._0_1_ = 9;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_3c;
    }
    else {
      local_1b4 = 0x4d51;
      local_1b2 = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_12c,&local_1b4);
      local_4._0_1_ = 10;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_12c;
    }
    break;
  case 2:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_202 = DAT_0065d424;
      local_204 = 0x4d60;
      pvVar2 = FUN_005f2fc0(local_9c,&local_204);
      local_4._0_1_ = 0xb;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_9c;
    }
    else {
      local_1d4 = 0x4d61;
      local_1d2 = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_114,&local_1d4);
      local_4._0_1_ = 0xc;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_114;
    }
    break;
  default:
    goto switchD_0052d96a_caseD_3;
  case 4:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1fa = DAT_0065d424;
      local_1fc = 0x4d70;
      pvVar2 = FUN_005f2fc0(local_54,&local_1fc);
      local_4._0_1_ = 0xd;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_54;
    }
    else {
      local_1bc = 0x4d71;
      local_1ba = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_fc,&local_1bc);
      local_4._0_1_ = 0xe;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_fc;
    }
    break;
  case 0x10:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1f2 = DAT_0065d424;
      local_1f4 = 0x4d80;
      pvVar2 = FUN_005f2fc0(local_84,&local_1f4);
      local_4._0_1_ = 0xf;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_84;
    }
    else {
      local_1cc = 0x4d81;
      local_1ca = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_e4,&local_1cc);
      local_4._0_1_ = 0x10;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_e4;
    }
    break;
  case 0x11:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1ea = DAT_0065d424;
      local_1ec = 0x4d82;
      pvVar2 = FUN_005f2fc0(local_24,&local_1ec);
      local_4._0_1_ = 0x11;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_24;
    }
    else {
      local_1ac = 0x4d83;
      local_1aa = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_198,&local_1ac);
      local_4._0_1_ = 0x12;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_198;
    }
    break;
  case 0x12:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_226 = DAT_0065d424;
      local_228 = 0x4d84;
      pvVar2 = FUN_005f2fc0(local_180,&local_228);
      local_4._0_1_ = 0x13;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_180;
    }
    else {
      local_220 = 0x4d85;
      local_21e = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_168,&local_220);
      local_4._0_1_ = 0x14;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_168;
    }
    break;
  case 0x13:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_216 = DAT_0065d424;
      local_218 = 0x4d86;
      pvVar2 = FUN_005f2fc0(local_150,&local_218);
      local_4._0_1_ = 0x15;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_150;
    }
    else {
      local_210 = 0x4d87;
      local_20e = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_138,&local_210);
      local_4._0_1_ = 0x16;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_138;
    }
    break;
  case 0x14:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_206 = DAT_0065d424;
      local_208 = 0x4d88;
      pvVar2 = FUN_005f2fc0(local_120,&local_208);
      local_4._0_1_ = 0x17;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_120;
    }
    else {
      local_200 = 0x4d89;
      local_1fe = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_108,&local_200);
      local_4._0_1_ = 0x18;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_108;
    }
    break;
  case 0x15:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1f6 = DAT_0065d424;
      local_1f8 = 0x4d8a;
      pvVar2 = FUN_005f2fc0(local_f0,&local_1f8);
      local_4._0_1_ = 0x19;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_f0;
    }
    else {
      local_1f0 = 0x4d8b;
      local_1ee = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_d8,&local_1f0);
      local_4._0_1_ = 0x1a;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_d8;
    }
    break;
  case 0x16:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1e6 = DAT_0065d424;
      local_1e8 = 0x4d8c;
      pvVar2 = FUN_005f2fc0(local_c0,&local_1e8);
      local_4._0_1_ = 0x1b;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_c0;
    }
    else {
      local_1e0 = 0x4d8d;
      local_1de = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_a8,&local_1e0);
      local_4._0_1_ = 0x1c;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_a8;
    }
    break;
  case 0x17:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1d6 = DAT_0065d424;
      local_1d8 = 0x4d8e;
      pvVar2 = FUN_005f2fc0(local_90,&local_1d8);
      local_4._0_1_ = 0x1d;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_90;
    }
    else {
      local_1d0 = 0x4d8f;
      local_1ce = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_78,&local_1d0);
      local_4._0_1_ = 0x1e;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_78;
    }
    break;
  case 0x18:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1c6 = DAT_0065d424;
      local_1c8 = 0x4d90;
      pvVar2 = FUN_005f2fc0(local_60,&local_1c8);
      local_4._0_1_ = 0x1f;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_60;
    }
    else {
      local_1c0 = 0x4d91;
      local_1be = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_48,&local_1c0);
      local_4._0_1_ = 0x20;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_48;
    }
    break;
  case 0x19:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_1b6 = DAT_0065d424;
      local_1b8 = 0x4d92;
      pvVar2 = FUN_005f2fc0(local_30,&local_1b8);
      local_4._0_1_ = 0x21;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_30;
    }
    else {
      local_1b0 = 0x4d93;
      local_1ae = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_18,&local_1b0);
      local_4._0_1_ = 0x22;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_18;
    }
    break;
  case 0x20:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_22a = DAT_0065d424;
      local_22c = 0x4d50;
      pvVar2 = FUN_005f2fc0(local_1a4,&local_22c);
      local_4._0_1_ = 1;
      FUN_005f3090(local_238,(int)pvVar2);
      puVar3 = local_1a4;
    }
    else {
      local_1a8 = 0x4d51;
      local_1a6 = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_18c,&local_1a8);
      local_4._0_1_ = 2;
      FUN_005f3090(local_238,(int)pvVar2);
      puVar3 = local_18c;
    }
    local_4._0_1_ = 0;
    FUN_005f2ff0(puVar3);
    bVar1 = FUN_005f34a0((int)local_238);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      local_1e2 = DAT_0065d424;
      local_1e4 = 0x4da2;
      pvVar2 = FUN_005f2fc0(local_174,&local_1e4);
      local_4._0_1_ = 4;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_174);
      FUN_005f30d0(param_1,(int)local_238);
      goto switchD_0052d96a_caseD_3;
    }
    local_224 = 0x4da1;
    local_222 = DAT_0065d424;
    pvVar2 = FUN_005f2fc0(local_cc,&local_224);
    local_4._0_1_ = 3;
    FUN_005f30d0(param_1,(int)pvVar2);
    local_4 = (uint)local_4._1_3_ << 8;
    puVar3 = local_cc;
    goto LAB_0052dbf2;
  case 0x21:
    if (((byte)*(undefined4 *)((int)this + -0xc) & 0xc0) == 0x40) {
      local_21a = DAT_0065d424;
      local_21c = 0x4d60;
      pvVar2 = FUN_005f2fc0(local_6c,&local_21c);
      local_4._0_1_ = 5;
      FUN_005f3090(local_238,(int)pvVar2);
      puVar3 = local_6c;
    }
    else {
      local_1c4 = 0x4d61;
      local_1c2 = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_15c,&local_1c4);
      local_4._0_1_ = 6;
      FUN_005f3090(local_238,(int)pvVar2);
      puVar3 = local_15c;
    }
    local_4._0_1_ = 0;
    FUN_005f2ff0(puVar3);
    bVar1 = FUN_005f34a0((int)local_238);
    if (CONCAT31(extraout_var_00,bVar1) == 0) {
      local_1da = DAT_0065d424;
      local_1dc = 0x4da2;
      pvVar2 = FUN_005f2fc0(local_144,&local_1dc);
      local_4._0_1_ = 8;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_144;
    }
    else {
      local_214 = 0x4da1;
      local_212 = DAT_0065d424;
      pvVar2 = FUN_005f2fc0(local_b4,&local_214);
      local_4._0_1_ = 7;
      FUN_005f30d0(param_1,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar3 = local_b4;
    }
LAB_0052dbf2:
    FUN_005f2ff0(puVar3);
    FUN_005f30d0(param_1,(int)local_238);
    goto switchD_0052d96a_caseD_3;
  }
  FUN_005f2ff0(puVar3);
switchD_0052d96a_caseD_3:
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_238);
  ExceptionList = local_c;
  return;
}

