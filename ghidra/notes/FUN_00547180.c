
bool __thiscall FUN_00547180(void *this,void *param_1,uint param_2)

{
  void *this_00;
  bool bVar1;
  int iVar2;
  int *piVar3;
  uint *puVar4;
  undefined3 extraout_var;
  undefined4 *puVar5;
  bool bVar6;
  uint local_24;
  uint local_20;
  undefined4 local_1c [2];
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647820;
  pvStack_c = ExceptionList;
  bVar1 = true;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  FUN_004ece30(&local_20);
  local_4._0_1_ = 1;
  FUN_004f4340(local_1c);
  this_00 = param_1;
  local_4 = CONCAT31(local_4._1_3_,2);
  puVar4 = (uint *)((int)param_1 + 0x38);
  iVar2 = FUN_004ece60(puVar4);
  if (iVar2 != 0) {
    FUN_004f6b70(this_00,&local_24);
    piVar3 = (int *)FUN_00504dc0(puVar4);
    bVar6 = piVar3 != (int *)0x0;
    if (piVar3 != (int *)0x0) {
      iVar2 = (**(code **)(*piVar3 + 0xc))(&local_20);
      if ((iVar2 == 0) || (!bVar6)) {
        bVar6 = false;
      }
      else {
        bVar6 = true;
      }
    }
    bVar1 = false;
    if (bVar6) {
      puVar4 = FUN_004025b0(this_00,(uint *)&param_1);
      local_4._0_1_ = 3;
      bVar1 = FUN_004f44b0(local_1c,puVar4,0);
      bVar1 = CONCAT31(extraout_var,bVar1) != 0;
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
    }
    puVar4 = FUN_004ece40(&local_24);
    if (puVar4 != (uint *)0x0) {
      puVar4 = FUN_004ece40(&local_20);
      if (puVar4 != (uint *)0x0) {
        puVar5 = FUN_004f4340(auStack_14);
        param_1 = (void *)0x41000001;
        local_4._0_1_ = 5;
        iVar2 = FUN_00547ba0(this,0,0,*(uint *)((int)this_00 + 0x24) >> 6 & 3,&local_24,
                             (uint *)&param_1,(int *)&local_24,&local_20,(int)local_1c,puVar5,
                             (void *)0x0,0,1,param_2);
        if ((iVar2 == 0) || (!bVar1)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        local_4._0_1_ = 4;
        FUN_00619730();
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_004f4380(auStack_14);
      }
    }
  }
  local_4._0_1_ = 1;
  FUN_004f4380(local_1c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return bVar1;
}

