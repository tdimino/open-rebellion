
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005d9b10(int param_1)

{
  int *piVar1;
  float fVar2;
  int *piVar3;
  int *piVar4;
  float *pfVar5;
  int iVar6;
  float10 fVar7;
  undefined8 uStack_1c;
  float fStack_14;
  float fStack_10;
  
  FUN_00595d00(*(uint *)(param_1 + 0x30),(int *)&uStack_1c,(int *)((int)&uStack_1c + 4));
  piVar3 = (int *)FUN_005c1080((uint)uStack_1c);
  piVar4 = (int *)FUN_005c1080(uStack_1c._4_4_);
  if (piVar3 == piVar4) {
    piVar4 = (int *)0x0;
  }
  if (piVar3 == (int *)0x0) {
    return 0;
  }
  (**(code **)(*piVar3 + 8))();
  (**(code **)(**(int **)(param_1 + 4) + 0x38))(*(int **)(param_1 + 4),0x3dcccccd);
  pfVar5 = (float *)(**(code **)(*piVar3 + 0x2c))();
  if (pfVar5 == (float *)0x0) {
    *(undefined4 *)(param_1 + 0x34) = 0;
  }
  else {
    fVar2 = pfVar5[3] - *pfVar5;
    if (pfVar5[3] - *pfVar5 <= pfVar5[4] - pfVar5[1]) {
      fVar2 = pfVar5[4] - pfVar5[1];
    }
    *(float *)(param_1 + 0x34) = fVar2;
  }
  if (piVar4 != (int *)0x0) {
    iVar6 = (**(code **)(*(int *)piVar4[0xb] + 0x60))((int *)piVar4[0xb],piVar3[0xb],&fStack_14);
    *(int *)(param_1 + 0x2c) = iVar6;
    if (iVar6 != 0) {
      FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
    }
    fVar7 = SQRT((float10)fStack_10 * (float10)fStack_10 + (float10)fStack_14 * (float10)fStack_14);
    uStack_1c = (double)fVar7;
    if ((float10)_DAT_0066d458 < fVar7) {
      iVar6 = (**(code **)(**(int **)(param_1 + 8) + 0xf4))
                        (*(int **)(param_1 + 8),piVar3[0xb],
                         (float)(((float10)fStack_14 * (float10)*(float *)(param_1 + 0x34)) / fVar7)
                         ,(float)(((float10)fStack_10 * (float10)*(float *)(param_1 + 0x34)) / fVar7
                                 ),*(float *)(param_1 + 0x34) + *(float *)(param_1 + 0x34));
      *(int *)(param_1 + 0x2c) = iVar6;
      if (iVar6 != 0) {
        FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
      }
      fVar7 = (float10)uStack_1c;
    }
    if ((piVar4 != (int *)0x0) && ((float10)_DAT_0066d458 < fVar7)) goto LAB_005d9cc2;
  }
  iVar6 = (**(code **)(**(int **)(param_1 + 8) + 0xf4))
                    (*(int **)(param_1 + 8),piVar3[0xb],-*(float *)(param_1 + 0x34),
                     *(undefined4 *)(param_1 + 0x34),
                     *(float *)(param_1 + 0x34) + *(float *)(param_1 + 0x34));
  *(int *)(param_1 + 0x2c) = iVar6;
  if (iVar6 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
  }
LAB_005d9cc2:
  piVar1 = *(int **)(param_1 + 8);
  if (piVar4 == (int *)0x0) {
    iVar6 = (**(code **)(*piVar1 + 0xf0))(piVar1,piVar3[0xb],0,0,0xbf800000,0,0x3f800000,0);
    *(int *)(param_1 + 0x2c) = iVar6;
  }
  else {
    iVar6 = (**(code **)(*piVar1 + 0x90))(piVar1,piVar4[0xb],*(undefined4 *)(param_1 + 0xc));
    *(int *)(param_1 + 0x2c) = iVar6;
  }
  if (iVar6 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
  }
  iVar6 = (**(code **)(**(int **)(param_1 + 4) + 0x40))
                    (*(int **)(param_1 + 4),*(undefined4 *)(param_1 + 0x38));
  *(int *)(param_1 + 0x2c) = iVar6;
  if (iVar6 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
  }
  iVar6 = (**(code **)(**(int **)(param_1 + 8) + 0x7c))
                    (*(int **)(param_1 + 8),*(undefined4 *)(param_1 + 0xc),param_1 + 0x14,
                     param_1 + 0x20);
  *(int *)(param_1 + 0x2c) = iVar6;
  if (iVar6 == 0) {
    return 1;
  }
  FUN_005d93f0(*(int *)(param_1 + 0x10),iVar6);
  return 1;
}

