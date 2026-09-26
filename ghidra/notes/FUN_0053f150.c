
int __cdecl FUN_0053f150(undefined4 param_1,int param_2,void *param_3,void *param_4)

{
  void *this;
  undefined4 *puVar1;
  void *this_00;
  uint *puVar2;
  int *piVar3;
  void *this_01;
  void *pvVar4;
  int local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 auStack_1c [2];
  undefined4 local_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar4 = param_4;
  puStack_8 = &LAB_00646900;
  pvStack_c = ExceptionList;
  local_28 = 1;
  local_24 = 1;
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_4,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar1 = FUN_004f4340(&local_14);
  this = param_3;
  local_4 = 1;
  FUN_004f43b0(param_3,puVar1);
  local_4 = 0xffffffff;
  FUN_004f4380(&local_14);
  FUN_004f4340(auStack_1c);
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  for (this_00 = (void *)thunk_FUN_005f5060(param_2); this_00 != (void *)0x0;
      this_00 = *(void **)((int)this_00 + 0x10)) {
    if (local_28 == 0) goto LAB_0053f31a;
    if (*(int *)((int)pvVar4 + 4) != -1) break;
    thunk_FUN_005f50a0(auStack_1c);
    puVar2 = FUN_00403040(this_00,&param_3);
    local_4._0_1_ = 3;
    piVar3 = (int *)FUN_0053efa0(puVar2);
    local_4._0_1_ = 2;
    FUN_00619730();
    if (piVar3 == (int *)0x0) {
      local_14 = 1;
      uStack_10 = 0x12;
      local_4._0_1_ = 4;
      FUN_00520580(pvVar4,&local_14);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    else if (*(int *)((int)pvVar4 + 4) == -1) {
      local_28 = (**(code **)(*piVar3 + 0x2c))
                           (param_1,*(undefined4 *)((int)this_00 + 0x1c),auStack_1c);
      this_01 = (void *)thunk_FUN_005f5060((int)auStack_1c);
      for (; this_01 != (void *)0x0; this_01 = *(void **)((int)this_01 + 0x10)) {
        puVar2 = FUN_00403040(this_01,&param_2);
        local_4._0_1_ = 5;
        pvVar4 = FUN_004f5940(this,puVar2);
        local_4._0_1_ = 2;
        FUN_00619730();
        if (pvVar4 == (void *)0x0) {
          puVar1 = FUN_00403040(this_01,&local_24);
          local_4._0_1_ = 6;
          FUN_004f44b0(this,puVar1,0);
          local_4._0_1_ = 2;
          FUN_00619730();
        }
        pvVar4 = param_4;
      }
    }
  }
  if (local_28 == 0) {
LAB_0053f31a:
    local_14 = 1;
    uStack_10 = 1;
    local_4._0_1_ = 7;
    FUN_00520580(pvVar4,&local_14);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(auStack_1c);
  ExceptionList = pvStack_c;
  return local_28;
}

