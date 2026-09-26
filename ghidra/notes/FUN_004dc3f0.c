
undefined1 __fastcall FUN_004dc3f0(int param_1)

{
  void *this;
  uint *this_00;
  bool bVar1;
  undefined1 uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d350;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x200,0,0,0,0,0,7,(void *)0x1);
  this = (void *)(param_1 + 0x68);
  FUN_00435790(this,(int)pvVar3);
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x200,0,0,0,0,0,9,(void *)0x1);
  FUN_00435790(this,(int)pvVar3);
  puVar4 = FUN_004357b0(this,&local_14);
  this_00 = (uint *)(param_1 + 0x40);
  local_4 = 0;
  FUN_004f26d0(this_00,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this_00,0x100,0,1,0x3e00003,0,0,0xf,(void *)0x1);
  FUN_00435790(this,(int)pvVar3);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this_00,0x100,0,1,0x3e00003,0,0,0,(void *)0x1);
  FUN_00435790(this,(int)pvVar3);
  puVar4 = FUN_004357b0(this,&local_14);
  local_4 = 1;
  FUN_004f26d0(this_00,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  local_14 = 0x90;
  local_10 = 0x98;
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x44),0x400,0,0,0,0,0,
                          2);
    FUN_00435790(this,(int)puVar4);
    puVar4 = FUN_004357b0(this,&local_14);
    local_4 = 2;
    FUN_004f26d0(this_00,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this);
    puVar4 = FUN_0041a400(*(void **)(param_1 + 0x2c),this_00,(undefined4 *)(param_1 + 0x44),0x20,0,0
                          ,0,0,0,(undefined4 *)&DAT_00000002);
    FUN_00435790(this,(int)puVar4);
    puVar4 = FUN_004357b0(this,&local_14);
    local_4 = 3;
    FUN_004f26d0(this_00,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this);
  }
  local_14 = 0x90;
  local_10 = 0x98;
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

