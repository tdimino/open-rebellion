
uint __thiscall FUN_00554f80(void *this,int *param_1,uint *param_2,uint *param_3,int *param_4)

{
  uint uVar1;
  void *pvVar2;
  int iVar3;
  uint *puVar4;
  int *piVar5;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  int *local_34;
  int *local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006493b8;
  pvStack_c = ExceptionList;
  local_40 = 1;
  local_3c = 1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_4,&local_40);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0058af30(&local_38);
  local_4 = 1;
  uVar1 = FUN_00554150(this,param_2,(int)&local_38);
  if (local_34 != (int *)0x0) {
    local_30 = local_34;
  }
  while (((uVar1 != 0 && (*local_30 != 0)) && (param_4[1] != -1))) {
    FUN_004ffef0(local_2c,*local_30,3,(uint)param_1[9] >> 6 & 3);
    local_4 = CONCAT31(local_4._1_3_,2);
    FUN_00513120((int)local_2c);
    while (((uVar1 != 0 && (local_10 != 0)) && (param_4[1] != -1))) {
      iVar3 = 0;
      puVar4 = param_3;
      piVar5 = param_4;
      pvVar2 = (void *)FUN_0052bed0((int)local_2c);
      uVar1 = FUN_005571c0(param_1,pvVar2,iVar3,puVar4,piVar5);
      FUN_005130d0((int)local_2c);
    }
    local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_004fff70(local_2c);
    if (*local_30 != 0) {
      local_30 = local_30 + 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_0058af70(&local_38);
  ExceptionList = pvStack_c;
  return uVar1;
}

