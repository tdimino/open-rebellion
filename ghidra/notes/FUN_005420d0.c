
bool __thiscall
FUN_005420d0(void *this,int *param_1,int param_2,int param_3,int *param_4,void *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *this_00;
  undefined4 *puVar3;
  undefined4 uStack_48;
  int iStack_44;
  int aiStack_40 [2];
  int aiStack_38 [2];
  int aiStack_30 [2];
  int aiStack_28 [2];
  uint auStack_20 [4];
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647028;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_4 + 4))();
  uStack_48 = 1;
  iStack_44 = -1;
  uStack_4 = 0;
  FUN_00520580(param_5,&uStack_48);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(aiStack_30);
  uStack_4 = 1;
  FUN_004f4340(aiStack_38);
  uStack_4._0_1_ = 2;
  FUN_004f4340(aiStack_40);
  uStack_4._0_1_ = 3;
  FUN_004f4340(aiStack_28);
  uStack_4._0_1_ = 4;
  bVar1 = FUN_0054bb90(param_1,param_2,param_3,aiStack_30,aiStack_38,aiStack_40,aiStack_28,param_5);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  FUN_00568fe0(auStack_20);
  uStack_4._0_1_ = 5;
  if (*(int *)((int)param_5 + 4) == -1) {
    iVar2 = FUN_005830a0((int)aiStack_30,(int)aiStack_38,(int)aiStack_40,auStack_20,param_5);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (*(int *)((int)param_5 + 4) == -1) {
      for (this_00 = (void *)FUN_0051cad0(0x40); this_00 != (void *)0x0;
          this_00 = *(void **)((int)this_00 + 0x10)) {
        if (*(int *)((int)this_00 + 0x5c) == 0) {
          uStack_48 = 1;
          iStack_44 = -1;
          uStack_4 = CONCAT31(uStack_4._1_3_,6);
          iVar2 = FUN_0054c440(this,uStack_10,aiStack_30,aiStack_38,aiStack_40,auStack_20,this_00,
                               &uStack_48);
          if ((iVar2 == 0) || (!bVar1)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          if (iStack_44 == -1) {
            FUN_00402e40(this_00,(uint *)&stack0xffffffa4);
            puVar3 = FUN_004f42a0(param_4);
            bVar1 = puVar3 != (undefined4 *)0x0;
          }
          uStack_4._0_1_ = 5;
          FUN_00619730();
        }
      }
    }
  }
  uStack_4._0_1_ = 4;
  FUN_00619730();
  uStack_4._0_1_ = 3;
  FUN_004f4380(aiStack_28);
  uStack_4._0_1_ = 2;
  FUN_004f4380(aiStack_40);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_004f4380(aiStack_38);
  uStack_4 = 0xffffffff;
  FUN_004f4380(aiStack_30);
  ExceptionList = pvStack_c;
  return bVar1;
}

