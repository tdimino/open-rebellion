
uint FUN_00556430(int *param_1,uint *param_2,uint *param_3,void *param_4)

{
  uint uVar1;
  uint uVar2;
  int *this;
  bool bVar3;
  int iVar4;
  int *piVar5;
  uint *puVar6;
  uint *puVar7;
  undefined3 extraout_var;
  uint uVar8;
  void *pvVar9;
  void *pvStack_18;
  void *pvStack_14;
  uint uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  this = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00649660;
  pvStack_c = ExceptionList;
  uVar8 = 1;
  ExceptionList = &pvStack_c;
  iVar4 = (**(code **)(*param_1 + 0x38))();
  if ((iVar4 != 0) && ((this[0x14] & 0x800U) == 0)) {
    piVar5 = (int *)FUN_00504dc0(param_2);
    if (piVar5 == (int *)0x0) {
      uVar8 = 0;
      puVar6 = param_3;
    }
    else {
      puVar6 = (uint *)FUN_00504dc0(param_3);
      uVar8 = (uint)(puVar6 != (uint *)0x0);
    }
    if ((((uVar8 != 0) && (uVar8 = FUN_00555540(piVar5,(int *)&pvStack_18), uVar8 != 0)) &&
        (uVar8 = FUN_00555540((int *)puVar6,(int *)&pvStack_14), uVar8 != 0)) &&
       ((pvStack_18 != (void *)0x0 && (pvStack_14 != (void *)0x0)))) {
      puVar6 = FUN_004025b0(pvStack_14,&uStack_10);
      uStack_4 = 0;
      puVar7 = FUN_004025b0(pvStack_18,(uint *)&param_1);
      uVar1 = *puVar7;
      uVar2 = *puVar6;
      FUN_00619730();
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (uVar1 != uVar2) {
        param_1 = (int *)0x0;
        bVar3 = FUN_00555d30(this,param_2,(int *)param_3,&param_1);
        pvVar9 = param_4;
        if (CONCAT31(extraout_var,bVar3) == 0) {
          uVar8 = 0;
        }
        else {
          uVar8 = FUN_004f7640(this,1,param_4);
          if ((uVar8 != 0) && (uVar8 = FUN_004f8010(this,(uint)param_1,pvVar9), uVar8 != 0)) {
            puVar6 = FUN_004025b0(pvStack_14,(uint *)&param_3);
            uStack_4 = 1;
            uVar8 = FUN_004f7120(this,(int *)puVar6,pvVar9);
            uStack_4 = 0xffffffff;
            FUN_00619730();
            if ((uVar8 != 0) && (uVar8 = FUN_004f7870(this,1,pvVar9), uVar8 != 0)) {
              iVar4 = FUN_004fd340();
              uVar8 = FUN_004f7390(this,iVar4 + (int)param_1,pvVar9);
            }
          }
        }
      }
    }
  }
  ExceptionList = pvStack_c;
  return uVar8;
}

