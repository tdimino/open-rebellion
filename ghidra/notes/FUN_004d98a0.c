
undefined4 __fastcall FUN_004d98a0(int param_1)

{
  void *this;
  uint *this_00;
  bool bVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ce10;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  puVar2 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x50),0x100,0,0,0,0,0,2)
  ;
  this = (void *)(param_1 + 0x40);
  FUN_00435790(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_004357b0(this,&local_18);
  local_4 = 0;
  this_00 = (uint *)(param_1 + 0x54);
  FUN_004f26d0(this_00,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  puVar2 = FUN_0041a400(*(void **)(param_1 + 0x2c),this_00,(undefined4 *)(param_1 + 0x50),0x80,0,0,
                        0x3e00003,0,0,(undefined4 *)&DAT_00000002);
  FUN_00435790(this,(int)puVar2);
  puVar2 = (undefined4 *)FUN_004357b0(this,&local_18);
  local_4 = 1;
  FUN_004f26d0(this_00,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  local_18 = 0x90;
  local_14 = 0x98;
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  uVar3 = 1;
  if (!bVar1) {
    uVar3 = local_10;
  }
  ExceptionList = local_c;
  return uVar3;
}

