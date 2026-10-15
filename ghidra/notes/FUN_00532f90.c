
int __thiscall FUN_00532f90(void *param_1,uint param_2,uint param_3,int param_4)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  undefined4 auStack_24 [3];
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00645740;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  bVar1 = FUN_0053a010((int)param_1);
  iVar3 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    FUN_005f2f50(auStack_18);
    uStack_4 = 0;
    FUN_005f2f50(auStack_24);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    iVar2 = FUN_00530a50(param_2,auStack_24);
    iVar3 = 0;
    if (iVar2 != 0) {
      iVar3 = FUN_00530a50(param_3,auStack_18);
    }
    if (iVar3 != 0) {
      FUN_004f8a50(param_1,s_SideVictoryConditionsNotif_006aa764,s_VictoryConditions_006aa780,
                   (int)auStack_18,(int)auStack_24,(char *)0x0,param_4);
    }
    iVar3 = FUN_0053fcf0(0x128,param_1,param_2,param_3,param_4);
    uStack_4 = uStack_4 & 0xffffff00;
    FUN_005f2ff0(auStack_24);
    uStack_4 = 0xffffffff;
    FUN_005f2ff0(auStack_18);
  }
  ExceptionList = pvStack_c;
  return iVar3;
}

