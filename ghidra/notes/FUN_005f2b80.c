
void __thiscall FUN_005f2b80(void *this,void *param_1)

{
  undefined4 *puVar1;
  int local_18;
  void *local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065554b;
  pvStack_c = ExceptionList;
  local_18 = 0;
  ExceptionList = &pvStack_c;
  local_14 = this;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&local_18);
  while (local_18 != 0) {
    local_18 = local_18 + -1;
    puVar1 = (undefined4 *)FUN_00618b70(0xf4);
    uStack_4 = 0;
    puStack_10 = puVar1;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      FUN_005f5590(puVar1);
      puVar1[7] = 0;
      puVar1[0xc] = 0;
      puVar1[0xd] = 0;
      puVar1[0xe] = 0;
      puVar1[0xf] = 0xbf800000;
      puVar1[0x10] = 0;
      puVar1[0x11] = 0;
      puVar1[0x12] = 0;
      puVar1[0x13] = 0xbf800000;
      puVar1[0x14] = 0;
      puVar1[0x15] = 0;
      puVar1[0x16] = 0;
      puVar1[0x17] = 0xbf800000;
      puVar1[0x18] = 0;
      puVar1[0x19] = 0;
      puVar1[0x1a] = 0;
      puVar1[0x1b] = 0xbf800000;
      puVar1[0x1f] = 0;
      puVar1[0x20] = 0;
      puVar1[0x21] = 0;
      puVar1[0x22] = 0xbf800000;
      puVar1[0x23] = 0;
      puVar1[0x24] = 0;
      puVar1[0x25] = 0;
      puVar1[0x26] = 0xbf800000;
      puVar1[0x27] = 0;
      puVar1[0x28] = 0;
      puVar1[0x29] = 0;
      puVar1[0x2a] = 0xbf800000;
      puVar1[0x2b] = 0;
      puVar1[0x2c] = 0;
      puVar1[0x2d] = 0;
      puVar1[0x2e] = 0xbf800000;
      puVar1[0x2f] = 0;
      puVar1[0x30] = 0;
      puVar1[0x31] = 0;
      puVar1[0x32] = 0xbf800000;
      puVar1[0x33] = 0;
      puVar1[0x34] = 0;
      puVar1[0x35] = 0;
      puVar1[0x36] = 0xbf800000;
      puVar1[0x37] = 0;
      puVar1[0x38] = 0;
      puVar1[0x39] = 0;
      puVar1[0x3a] = 0xbf800000;
      puVar1[8] = &PTR_FUN_0066c648;
      puVar1[9] = 0x3f800000;
      puVar1[0x1c] = 0;
      puVar1[0xc] = 0;
      puVar1[0xd] = 0;
      puVar1[0xe] = 0x3f800000;
      puVar1[0xf] = 0xbf800000;
      *puVar1 = &PTR_FUN_0066d968;
      puVar1[8] = &PTR_LAB_0066d948;
    }
    uStack_4 = 0xffffffff;
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)(puVar1[8] + 0x10))(param_1);
      FUN_005c61a0(local_14,puVar1);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

