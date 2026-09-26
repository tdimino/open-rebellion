
int __cdecl FUN_005187f0(uint param_1,int param_2,int *param_3,void *param_4)

{
  int *this;
  void *this_00;
  bool bVar1;
  int iVar2;
  uint *puVar3;
  int iVar4;
  uint uVar5;
  void *pvVar6;
  undefined3 extraout_var;
  uint uStack_30;
  uint uStack_2c;
  uint uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642b38;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_3 + 4))();
  this_00 = param_4;
  uStack_2c = 1;
  uStack_28 = 0xffffffff;
  uStack_4 = 0;
  FUN_00520580(param_4,&uStack_2c);
  uStack_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(auStack_14);
  uStack_4._0_1_ = 1;
  uStack_4._1_3_ = 0;
  iVar2 = FUN_0053f150(param_1,param_2,auStack_14,this_00);
  if (iVar2 != 0) {
    if (*(int *)((int)this_00 + 4) == -1) {
      for (param_4 = (void *)thunk_FUN_005f5060((int)auStack_14); param_4 != (void *)0x0;
          param_4 = *(void **)((int)param_4 + 0x10)) {
        if (iVar2 == 0) goto LAB_00518a1e;
        if (*(int *)((int)this_00 + 4) != -1) break;
        puVar3 = FUN_00403040(param_4,&param_2);
        uStack_4._0_1_ = 2;
        iVar4 = FUN_00506480(puVar3);
        uStack_4._0_1_ = 1;
        FUN_00619730();
        if (iVar4 == 0) {
          uStack_24 = 1;
          uStack_20 = 0x12;
          uStack_4._0_1_ = 3;
          FUN_00520580(this_00,&uStack_24);
          uStack_4._0_1_ = 1;
          FUN_00619730();
        }
        else {
          if ((*(int *)((int)this_00 + 4) == -1) && ((*(byte *)(iVar4 + 0x50) & 8) != 0)) {
            uStack_1c = 1;
            uStack_18 = 0x12;
            uStack_4._0_1_ = 4;
            FUN_00520580(this_00,&uStack_1c);
            uStack_4._0_1_ = 1;
            FUN_00619730();
          }
          if ((iVar4 != 0) && (*(int *)((int)this_00 + 4) == -1)) {
            this = *(int **)(iVar4 + 0x1c);
            uStack_2c = 0x90;
            uStack_28 = 0x98;
            uStack_4._0_1_ = 5;
            uVar5 = (**(code **)(*this + 4))();
            if ((uVar5 < uStack_2c) || (uStack_28 <= uVar5)) {
              iVar2 = 0;
            }
            else {
              iVar2 = 1;
            }
            uStack_4._0_1_ = 1;
            FUN_00619730();
            if (this != (int *)0x0) {
              puVar3 = FUN_004025b0(this,&param_1);
              uStack_4._0_1_ = 6;
              pvVar6 = FUN_004f5940(param_3,puVar3);
              uStack_4._0_1_ = 1;
              FUN_00619730();
              if (pvVar6 == (void *)0x0) {
                puVar3 = FUN_004025b0(this,&uStack_30);
                uStack_4._0_1_ = 7;
                bVar1 = FUN_004f44b0(param_3,puVar3,0);
                iVar2 = CONCAT31(extraout_var,bVar1);
                uStack_4._0_1_ = 1;
                FUN_00619730();
              }
            }
          }
        }
      }
    }
    if (iVar2 != 0) goto LAB_00518a45;
  }
LAB_00518a1e:
  uStack_1c = 1;
  uStack_18 = 1;
  uStack_4._0_1_ = 8;
  FUN_00520580(this_00,&uStack_1c);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_00619730();
LAB_00518a45:
  uStack_4 = 0xffffffff;
  FUN_004f4380(auStack_14);
  ExceptionList = pvStack_c;
  return iVar2;
}

