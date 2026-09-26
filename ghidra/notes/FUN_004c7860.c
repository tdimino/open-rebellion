
undefined4 __fastcall FUN_004c7860(int param_1)

{
  uint *this;
  bool bVar1;
  undefined4 *puVar2;
  int *this_00;
  void *pvVar3;
  uint *puVar4;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ad58;
  local_c = ExceptionList;
  local_18[2] = 0;
  ExceptionList = &local_c;
  puVar2 = FUN_0049d770(*(void **)(param_1 + 0x28),(undefined4 *)(param_1 + 0x40),0x22000,0,0,0,2);
  pvVar3 = (void *)(param_1 + 0x4c);
  FUN_00435790(pvVar3,(int)puVar2);
  puVar2 = FUN_004357b0(pvVar3,local_18);
  local_4 = 0;
  this = (uint *)(param_1 + 0x44);
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar3);
  puVar2 = FUN_0049da70(*(void **)(param_1 + 0x28),this,(undefined4 *)(param_1 + 0x40),0x104,0,0x800
                        ,0,(undefined4 *)&DAT_00000002);
  FUN_00435790(pvVar3,(int)puVar2);
  puVar2 = FUN_004357b0(pvVar3,local_18);
  local_4 = 1;
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar3);
  local_18[0] = 0x90;
  local_18[1] = 0x98;
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
      pvVar3 = (void *)FUN_00509670(this_00,2);
      if (pvVar3 != (void *)0x0) {
        puVar4 = FUN_004025b0(pvVar3,local_18);
        local_4 = 2;
        FUN_004f26d0(this,puVar4);
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = local_c;
        return 1;
      }
    }
  }
  ExceptionList = local_c;
  return local_18[2];
}

