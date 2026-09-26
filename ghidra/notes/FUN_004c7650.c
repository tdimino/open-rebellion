
undefined4 __fastcall FUN_004c7650(int param_1)

{
  void *this;
  uint *this_00;
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  undefined4 uVar4;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ad30;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  pvVar2 = FUN_0049d610(*(void **)(param_1 + 0x28),0x21000000,0,0,0,0x19,(void *)0x1);
  this = (void *)(param_1 + 0x4c);
  FUN_00435790(this,(int)pvVar2);
  puVar3 = FUN_004357b0(this,&local_18);
  local_4 = 0;
  this_00 = (uint *)(param_1 + 0x40);
  FUN_004f26d0(this_00,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(this);
  pvVar2 = FUN_0049da10(*(void **)(param_1 + 0x28),this_00,0x10000005,0,2,0,0x14,(void *)0x1);
  FUN_00435790(this,(int)pvVar2);
  puVar3 = FUN_004357b0(this,&local_18);
  local_4 = 1;
  FUN_004f26d0(this_00,puVar3);
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
  if (!bVar1) {
    pvVar2 = FUN_0049d610(*(void **)(param_1 + 0x28),0x1000000,0,0,0,2,&DAT_00000002);
    FUN_00435790(this,(int)pvVar2);
    puVar3 = FUN_004357b0(this,&local_18);
    local_4 = 2;
    FUN_004f26d0(this_00,puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this);
    pvVar2 = FUN_0049da10(*(void **)(param_1 + 0x28),this_00,0x10000004,0,2,0,0x14,&DAT_00000002);
    FUN_00435790(this,(int)pvVar2);
    puVar3 = FUN_004357b0(this,&local_18);
    local_4 = 3;
    FUN_004f26d0(this_00,puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this);
  }
  local_18 = 0x90;
  local_14 = 0x98;
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  uVar4 = 1;
  if (!bVar1) {
    uVar4 = local_10;
  }
  ExceptionList = local_c;
  return uVar4;
}

