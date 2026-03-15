
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void * __thiscall FUN_005ea030(void *this,uint param_1)

{
  float fVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006552bf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f52c0((undefined4 *)this);
  *(undefined ***)this = &PTR_FUN_0066bdd0;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0xbf800000;
  FUN_0059faf0((undefined4 *)((int)this + 0x2c));
  FUN_0059faf0((undefined4 *)((int)this + 0x3c));
  FUN_0059faf0((undefined4 *)((int)this + 0x4c));
  FUN_0059faf0((undefined4 *)((int)this + 0x68));
  FUN_0059faf0((undefined4 *)((int)this + 0x78));
  FUN_0059faf0((undefined4 *)((int)this + 0x88));
  FUN_0059faf0((undefined4 *)((int)this + 0x98));
  FUN_0059faf0((undefined4 *)((int)this + 0xa8));
  FUN_0059faf0((undefined4 *)((int)this + 0xb8));
  FUN_0059faf0((undefined4 *)((int)this + 200));
  *(undefined ***)((int)this + 0xc) = &PTR_FUN_0066c648;
  FUN_005a33b0((undefined4 *)((int)this + 0x3c),(undefined4 *)&stack0x00000008);
  FUN_0059fb10((undefined4 *)((int)this + 0x1c),0,0,0x3f800000);
  *(undefined4 *)((int)this + 0x10) = 0x3f800000;
  *(undefined4 *)((int)this + 0x5c) = 0;
  local_4._0_1_ = 1;
  FUN_005c6150((undefined4 *)((int)this + 0xe0));
  *(undefined4 *)((int)this + 0xe0) = &PTR_FUN_0066d938;
  local_4._0_1_ = 2;
  FUN_005c6150((undefined4 *)((int)this + 0xec));
  *(undefined4 *)((int)this + 0xec) = &PTR_FUN_0066d938;
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_005f4ed0((undefined4 *)((int)this + 0xf8));
  *(undefined4 *)((int)this + 0xf8) = &PTR_FUN_0066d928;
  *(undefined4 *)((int)this + 0x104) = 0;
  *(undefined4 *)((int)this + 0x108) = 0;
  *(undefined4 *)((int)this + 0x10c) = 0;
  *(undefined4 *)((int)this + 0x110) = 0xbf800000;
  *(undefined4 *)((int)this + 0x114) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x11c) = 0;
  *(undefined4 *)((int)this + 0x120) = 0xbf800000;
  *(undefined ***)this = &PTR_FUN_0066d8e0;
  *(undefined ***)((int)this + 0xc) = &PTR_LAB_0066d8c0;
  fVar1 = _DAT_0066d848 * _DAT_0066d864;
  *(undefined4 *)((int)this + 0x134) = 0;
  *(float *)((int)this + 0x13c) = fVar1;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 300) = 0;
  *(undefined4 *)((int)this + 0x124) = 0;
  *(undefined4 *)((int)this + 0x128) = 0x40800000;
  *(uint *)((int)this + 0x100) = param_1;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  if (param_1 < 1000000) {
    *(undefined4 *)((int)this + 0x24) = 0x3f800000;
    *(undefined4 *)((int)this + 0x28) = 0xbf800000;
    fVar1 = _DAT_0066d838 * _DAT_0066d86c;
  }
  else {
    *(undefined4 *)((int)this + 0x24) = 0xbf800000;
    *(undefined4 *)((int)this + 0x28) = 0xbf800000;
    fVar1 = _DAT_0066d838 * _DAT_0066d868;
  }
  *(float *)((int)this + 0x138) = fVar1;
  ExceptionList = local_c;
  return this;
}

