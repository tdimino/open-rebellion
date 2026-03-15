
uint __thiscall
FUN_00528d30(void *this,uint *param_1,int param_2,int param_3,void *param_4,int *param_5)

{
  int *piVar1;
  void *pvVar2;
  bool bVar3;
  void *pvVar4;
  uint *puVar5;
  undefined3 extraout_var;
  int iVar6;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar7;
  int *piVar8;
  undefined3 extraout_var_02;
  int iVar9;
  int local_38;
  uint local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_4;
  puStack_8 = &LAB_006446e8;
  local_c = ExceptionList;
  bVar3 = true;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_5 = 0;
  pvVar4 = (void *)FUN_0051cab0(param_1);
  param_1 = (uint *)0x0;
  if (pvVar4 != (void *)0x0) {
    puVar5 = (uint *)FUN_00402e40(pvVar4,(uint *)&param_4);
    local_34 = 0x10;
    local_30 = 0x40;
    local_4 = 1;
    if ((*puVar5 >> 0x18 < 0x10) || (0x3f < *puVar5 >> 0x18)) {
      bVar3 = false;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((bVar3) && (param_1 = (uint *)FUN_0053b860((int)pvVar4), -1 < param_2)) {
      param_1 = (uint *)(param_2 * (int)param_1);
      if (param_3 != 0) {
        param_1 = (uint *)((int)param_1 - (*(int *)((int)this + 0x60) + *(int *)((int)this + 0x5c)))
        ;
      }
      if ((int)param_1 < 0) {
        param_1 = (uint *)0x0;
      }
    }
  }
  local_38 = 0;
  bVar3 = FUN_00528960(this,pvVar2,&local_38);
  if ((CONCAT31(extraout_var,bVar3) == 0) || (param_4 = (void *)0x1, pvVar4 == (void *)0x0)) {
    param_4 = (void *)0x0;
  }
  if ((*(int *)((int)pvVar2 + 4) == -1) && (local_38 != 0)) {
    iVar6 = FUN_0053e160((int)param_1 * DAT_00661a88,local_38);
    iVar9 = 0;
    param_2 = 0;
    bVar3 = FUN_00528b30(this,iVar6,param_3,pvVar2,&param_2);
    pvVar4 = param_4;
    if ((CONCAT31(extraout_var_00,bVar3) == 0) || (param_4 = (void *)0x1, pvVar4 == (void *)0x0)) {
      param_4 = (void *)0x0;
    }
    if (*(int *)((int)pvVar2 + 4) == -1) {
      piVar8 = param_5;
      if (param_2 < (int)param_1) {
        iVar7 = FUN_0052bed0((int)this);
        if (iVar7 != 0) {
          FUN_0052cda0(local_2c,iVar7);
          local_4 = 2;
          FUN_00513120((int)local_2c);
          iVar7 = FUN_0052bed0((int)local_2c);
          while (iVar7 != 0) {
            piVar8 = (int *)FUN_0052bed0((int)local_2c);
            iVar7 = FUN_0052bde0(this,piVar8);
            if (iVar7 != 0) {
              iVar7 = FUN_0052bed0((int)local_2c);
              local_34 = *(uint *)(iVar7 + 0x60) & 1;
              if (local_34 == 0) {
                if (iVar9 != 0) {
                  iVar7 = FUN_0052bed0((int)local_2c);
                  iVar7 = FUN_00520b70(iVar7);
                  if (iVar7 <= iVar9) goto LAB_00528fdb;
                }
                iVar9 = FUN_0052bed0((int)local_2c);
                iVar9 = FUN_00520b70(iVar9);
              }
            }
LAB_00528fdb:
            FUN_005130d0((int)local_2c);
            iVar7 = FUN_0052bed0((int)local_2c);
          }
          local_4 = 0xffffffff;
          FUN_0052cec0(local_2c);
        }
        iVar9 = iVar9 + iVar6;
        piVar8 = param_5;
        while ((piVar1 = param_5, iVar6 <= iVar9 && (piVar8 = piVar1, *piVar1 == 0))) {
          if (*(int *)((int)pvVar2 + 4) != -1) {
            ExceptionList = local_c;
            return (uint)param_4;
          }
          param_5 = piVar1;
          bVar3 = FUN_00528b30(this,iVar6,param_3,pvVar2,&param_2);
          pvVar4 = param_4;
          if ((CONCAT31(extraout_var_02,bVar3) == 0) ||
             (param_4 = (void *)0x1, pvVar4 == (void *)0x0)) {
            param_4 = (void *)0x0;
          }
          if ((*(int *)((int)pvVar2 + 4) == -1) && ((int)param_1 <= param_2)) {
            *piVar1 = iVar6;
          }
          iVar6 = iVar6 + 1;
        }
        if ((*(int *)((int)pvVar2 + 4) == -1) && (*piVar8 == 0)) {
          *piVar8 = iVar9;
        }
      }
      else {
        while (((-1 < iVar6 && (*piVar8 == 0)) && (*(int *)((int)pvVar2 + 4) == -1))) {
          param_5 = piVar8;
          bVar3 = FUN_00528b30(this,iVar6,param_3,pvVar2,&param_2);
          pvVar4 = param_4;
          if ((CONCAT31(extraout_var_01,bVar3) == 0) ||
             (param_4 = (void *)0x1, pvVar4 == (void *)0x0)) {
            param_4 = (void *)0x0;
          }
          if ((*(int *)((int)pvVar2 + 4) == -1) && (param_2 < (int)param_1)) {
            *piVar8 = iVar6 + 1;
          }
          iVar6 = iVar6 + -1;
          piVar8 = param_5;
        }
      }
    }
  }
  ExceptionList = local_c;
  return (uint)param_4;
}

