
int __thiscall
FUN_00552b10(void *this,void *param_1,int param_2,int param_3,int *param_4,void *param_5)

{
  int iVar1;
  bool bVar2;
  uint *puVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined3 extraout_var_00;
  undefined4 *puVar5;
  undefined3 extraout_var_01;
  void *local_8c;
  void *local_88;
  uint local_84 [2];
  undefined4 local_7c [7];
  undefined4 local_60 [7];
  undefined4 local_44 [7];
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648eab;
  local_c = ExceptionList;
  local_8c = (void *)0x0;
  local_88 = (void *)0x0;
  ExceptionList = &local_c;
  puVar3 = FUN_004025b0(param_1,local_84);
  local_4 = 0;
  bVar2 = FUN_005530d0(this,puVar3,(int *)&local_8c,&local_88);
  iVar4 = CONCAT31(extraout_var,bVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0054fa40(local_7c);
  local_4 = 1;
  FUN_0054fa40(local_44);
  local_4._0_1_ = 2;
  FUN_0054fa40(local_60);
  local_4._0_1_ = 3;
  FUN_0054fa40(local_28);
  local_4 = CONCAT31(local_4._1_3_,4);
  if (iVar4 != 0) {
    iVar4 = FUN_00550c90(local_8c,param_2,param_3,local_44,local_28);
  }
  if (iVar4 != 0) {
    iVar4 = FUN_00550de0(local_88,param_2,param_3,local_60);
    if (iVar4 != 0) {
      bVar2 = FUN_0054fbb0(local_44,(int)local_60);
      if (CONCAT31(extraout_var_00,bVar2) == 0) {
        puVar5 = local_44;
      }
      else {
        puVar5 = local_60;
      }
      FUN_0054fae0(local_7c,(int)puVar5);
      if (iVar4 != 0) {
        puVar5 = FUN_0054fc40(local_7c,local_84);
        iVar1 = puVar5[1];
        FUN_00619730();
        if (iVar1 == -1) {
          FUN_0054fae0(local_7c,(int)local_28);
        }
        if (*param_4 != 0) {
          bVar2 = FUN_0054fbb0(local_7c,(int)param_5);
          if (CONCAT31(extraout_var_01,bVar2) == 0) goto LAB_00552ca2;
        }
        *param_4 = (int)local_8c;
        FUN_0054fae0(param_5,(int)local_7c);
      }
    }
  }
LAB_00552ca2:
  local_4._0_1_ = 3;
  FUN_0054fa90(local_28);
  local_4._0_1_ = 2;
  FUN_0054fa90(local_60);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_0054fa90(local_44);
  local_4 = 0xffffffff;
  FUN_0054fa90(local_7c);
  ExceptionList = local_c;
  return iVar4;
}

