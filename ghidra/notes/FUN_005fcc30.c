
uint __thiscall
FUN_005fcc30(void *this,uint *param_1,int param_2,int param_3,int param_4,uint param_5,
            undefined4 *param_6,uint param_7)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  uint uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  longlong lVar14;
  int local_18;
  int local_14;
  
  iVar2 = FUN_005fc0e0((int *)this);
  if (iVar2 < param_4) {
    return 0;
  }
  bVar1 = FUN_005fc040((int *)param_1);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    return 0;
  }
  iVar2 = FUN_005fc110((int *)param_1);
  if (iVar2 != 0) {
    return 0;
  }
  uVar3 = FUN_005fc100(param_1);
  if (uVar3 != 0x18) {
    iVar2 = FUN_005fc110((int *)this);
    if (iVar2 == 1) {
      uVar3 = FUN_005fd1e0(this,param_1,param_2,param_3,param_4,param_5,(int)param_6,param_7);
      return uVar3;
    }
    iVar2 = FUN_00583c40((int)param_1);
    iVar4 = FUN_00583c40((int)this);
    iVar5 = FUN_005fc0f0((int *)param_1);
    iVar6 = FUN_005fc0e0((int *)param_1);
    uVar3 = FUN_005fc9c0(param_1);
    uVar7 = FUN_005fc9c0((uint *)this);
    if (iVar5 < 0) {
      local_18 = 1;
      iVar5 = -iVar5;
    }
    else {
      local_18 = -1;
    }
    iVar8 = FUN_005fc0f0((int *)this);
    if (iVar8 < 0) {
      iVar8 = -iVar8;
      local_14 = 1;
    }
    else {
      local_14 = -1;
    }
    if ((int)param_5 <= iVar8) {
      if (param_6 == (undefined4 *)0x0) {
        if (param_2 < 0) {
          param_2 = 0;
        }
        param_6 = (undefined4 *)(iVar6 - param_2);
      }
      if (param_7 == 0) {
        if (param_3 < 0) {
          param_3 = 0;
        }
        param_7 = iVar5 - param_3;
      }
      if (param_2 < 0) {
        param_6 = (undefined4 *)((int)param_6 + param_2);
        param_4 = param_4 - param_2;
      }
      if (param_3 < 0) {
        param_7 = param_7 + param_3;
        param_5 = param_5 - param_3;
        param_3 = 0;
      }
      uVar9 = param_5;
      if ((param_4 < 0) && ((int)param_6 + param_4 < 1)) {
        return 0;
      }
      if ((int)param_5 < 0) {
        param_7 = param_7 + param_5;
        if ((int)param_7 < 1) {
          return 0;
        }
        param_5 = 0;
        param_3 = param_3 - uVar9;
      }
      if ((int)(iVar8 - param_5) < (int)param_7) {
        param_7 = iVar8 - param_5;
      }
      iVar5 = FUN_005fc9e0(param_1,param_3);
      iVar6 = FUN_005fc9e0(this,param_5);
      uVar9 = FUN_005fc100((uint *)this);
      switch(uVar9) {
      case 1:
        break;
      default:
        break;
      case 4:
        break;
      case 0x18:
        break;
      case 0x20:
      }
      if (0 < (int)param_7) {
        lVar13 = __ftol();
        lVar14 = __ftol();
        param_6 = (undefined4 *)(uVar3 * iVar5 + iVar2 + (int)lVar14);
        lVar14 = __ftol();
        param_5 = param_7;
        puVar11 = (undefined4 *)(uVar7 * iVar6 + iVar4 + (int)lVar14);
        do {
          puVar10 = puVar11;
          puVar12 = param_6;
          for (uVar9 = (uint)lVar13 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
            *puVar12 = *puVar10;
            puVar10 = puVar10 + 1;
            puVar12 = puVar12 + 1;
          }
          puVar11 = (undefined4 *)((int)puVar11 + uVar7 * local_14);
          for (uVar9 = (uint)lVar13 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
            *(undefined1 *)puVar12 = *(undefined1 *)puVar10;
            puVar10 = (undefined4 *)((int)puVar10 + 1);
            puVar12 = (undefined4 *)((int)puVar12 + 1);
          }
          param_6 = (undefined4 *)((int)param_6 + uVar3 * local_18);
          param_5 = param_5 - 1;
        } while (param_5 != 0);
      }
      FUN_005fd170(param_1);
      return (uint)(0 < (int)param_7);
    }
    return 0;
  }
  FUN_0060ea50(this,param_1,param_2,param_3,param_4,param_5,(int)param_6,param_7);
  return 1;
}

