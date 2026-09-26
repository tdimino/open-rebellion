
uint FUN_00555f90(void *param_1,uint *param_2,void *param_3,int *param_4)

{
  void *pvVar1;
  bool bVar2;
  void *pvVar3;
  uint *puVar4;
  uint *puVar5;
  undefined3 extraout_var;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006495c8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  pvVar3 = (void *)FUN_0051cab0(param_2);
  pvVar1 = param_1;
  param_3 = (void *)(uint)(pvVar3 != (void *)0x0);
  if (pvVar3 != (void *)0x0) {
    puVar4 = FUN_004591b0(param_1,&param_1);
    local_4 = 1;
    puVar5 = FUN_004025b0(pvVar1,(uint *)&param_2);
    local_4._0_1_ = 2;
    bVar2 = FUN_00555d90(pvVar3,*(uint *)((int)pvVar1 + 0x24) >> 4 & 3,puVar5,puVar4,param_4);
    pvVar1 = param_3;
    if ((CONCAT31(extraout_var,bVar2) == 0) || (param_3 = (void *)0x1, pvVar1 == (void *)0x0)) {
      param_3 = (void *)0x0;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return (uint)param_3;
}

