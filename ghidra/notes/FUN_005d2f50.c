
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005d2f50(int param_1)

{
  int *piVar1;
  float *pfVar2;
  int iVar3;
  float unaff_ESI;
  float local_1c;
  float fStack_18;
  float fStack_14;
  float fStack_10;
  int *piStack_8;
  float fStack_4;
  
  piVar1 = *(int **)(*(int *)(DAT_006bcbd4 + 0x638) + 8);
  pfVar2 = &local_1c;
  iVar3 = (**(code **)(*piVar1 + 0x60))(piVar1,*(undefined4 *)(param_1 + 0x2c));
  FUN_005d8b00(iVar3);
  fStack_18 = 0.0;
  fStack_14 = unaff_ESI;
  if ((_DAT_0066d178 != _DAT_0066d17c) &&
     (fStack_10 = SQRT((float)pfVar2 * (float)pfVar2 + unaff_ESI * unaff_ESI + 0.0),
     _DAT_0066d180 < fStack_10)) {
    fStack_14 = _DAT_0066d17c / fStack_10;
    pfVar2 = (float *)(fStack_14 * (float)pfVar2);
    fStack_18 = fStack_14 * 0.0;
    fStack_14 = fStack_14 * unaff_ESI;
  }
  local_1c = (float)pfVar2 * fStack_4;
  fStack_18 = fStack_18 * fStack_4;
  fStack_14 = fStack_14 * fStack_4;
  iVar3 = (**(code **)(*piStack_8 + 0xf4))
                    (piStack_8,*(undefined4 *)(param_1 + 0x2c),local_1c,fStack_18,fStack_14,0);
  FUN_005d8b00(iVar3);
  return;
}

