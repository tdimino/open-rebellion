
undefined4 __fastcall FUN_004c6d90(int param_1)

{
  uint *this;
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  int *this_00;
  void *pvVar4;
  uint *puVar5;
  undefined4 uVar6;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ac08;
  local_c = ExceptionList;
  uVar6 = 0;
  ExceptionList = &local_c;
  pvVar2 = FUN_0049d610(*(void **)(param_1 + 0x28),0x44000,0,0,0,10,&DAT_00000002);
  pvVar4 = (void *)(param_1 + 0x48);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,local_14);
  this = (uint *)(param_1 + 0x40);
  local_4 = 0;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  pvVar2 = FUN_0049da10(*(void **)(param_1 + 0x28),this,0x204,0,0x1000,0,7,&DAT_00000002);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,local_14);
  local_4 = 1;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  local_14[0] = 0x90;
  local_14[1] = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    this_00 = FUN_004f3220(*(int *)(param_1 + 0x2c),this);
    if (this_00 != (int *)0x0) {
      pvVar4 = (void *)FUN_00509670(this_00,1);
      if (pvVar4 != (void *)0x0) {
        puVar5 = FUN_004025b0(pvVar4,local_14);
        local_4 = 2;
        FUN_004f26d0(this,puVar5);
        local_4 = 0xffffffff;
        FUN_00619730();
        uVar6 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return uVar6;
}

