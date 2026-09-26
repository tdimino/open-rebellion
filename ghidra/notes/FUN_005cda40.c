
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int __thiscall FUN_005cda40(void *this,int param_1,float *param_2)

{
  int iVar1;
  void *this_00;
  float *pfVar2;
  int iVar3;
  int iVar4;
  float10 fVar5;
  undefined4 *puVar6;
  int iVar7;
  float local_20;
  float local_1c;
  float local_18;
  float local_14;
  undefined4 local_10 [4];
  
  iVar1 = FUN_005cec50((int)this);
  iVar4 = 0;
  local_1c = param_2[1];
  local_20 = *param_2;
  local_18 = param_2[2];
  local_14 = param_2[3];
  if (((local_20 == DAT_006bc940) && (local_1c == DAT_006bc944)) && (local_18 == DAT_006bc948)) {
    puVar6 = local_10;
    iVar7 = param_1;
    this_00 = (void *)FUN_005aaf90();
    pfVar2 = (float *)FUN_005a7490(this_00,puVar6,iVar7);
    local_20 = *pfVar2;
    local_1c = pfVar2[1];
    local_18 = pfVar2[2];
    local_14 = pfVar2[3];
  }
  if (local_14 != _DAT_0066d040) {
    fVar5 = FUN_005b50b0(&local_20);
    if ((float10)_DAT_0066d044 < fVar5) {
      fVar5 = (float10)_DAT_0066d040 / fVar5;
      local_20 = (float)(fVar5 * (float10)local_20);
      local_1c = (float)(fVar5 * (float10)local_1c);
      local_18 = (float)(fVar5 * (float10)local_18);
    }
    local_14 = 1.0;
  }
  param_2 = (float *)0xff7fffff;
  for (; iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    iVar7 = *(int *)(iVar1 + 0x18);
    iVar3 = iVar7;
    FUN_005aaf90();
    iVar3 = FUN_005ad870(iVar3);
    if (((iVar3 != 0) && (*(int *)(*(int *)(iVar1 + 0x18) + 0x170) != 6)) &&
       ((*(int *)(iVar7 + 0x168) != param_1 &&
        (pfVar2 = (float *)(*(float *)(iVar7 + 0x30) * local_20 +
                           *(float *)(iVar7 + 0x38) * local_18 + *(float *)(iVar7 + 0x34) * local_1c
                           ), (float)param_2 < (float)pfVar2)))) {
      iVar4 = iVar7;
      param_2 = pfVar2;
    }
  }
  return iVar4;
}

