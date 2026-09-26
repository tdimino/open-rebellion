
uint __thiscall FUN_005b4c60(void *this,float param_1)

{
  float fVar1;
  float *pfVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  
  uVar5 = 4;
  pfVar2 = (float *)((int)this + 0x198);
  iVar4 = 4;
  uVar3 = 0;
  do {
    pfVar2[0x15] = 0.0;
    pfVar2[-2] = 0.0;
    pfVar2[-4] = 0.0;
    pfVar2[-3] = 0.0;
    if ((pfVar2[-6] != 0.0) && (param_1 == 0.0)) {
      pfVar2[-2] = (float)(int)pfVar2[-6] * pfVar2[2] * *pfVar2;
    }
    if (pfVar2[-7] != 0.0) {
      if (param_1 == 0.0) {
        fVar1 = pfVar2[7];
      }
      else {
        fVar1 = pfVar2[6];
      }
      pfVar2[-4] = (float)(int)pfVar2[-7] * pfVar2[5] * fVar1;
    }
    if (pfVar2[-5] != 0.0) {
      if (param_1 == 0.0) {
        fVar1 = pfVar2[0xc] * pfVar2[10];
      }
      else {
        fVar1 = pfVar2[10] * pfVar2[0xb];
      }
      pfVar2[-3] = (float)(int)pfVar2[-5] * fVar1;
    }
    pfVar2 = pfVar2 + 0x1d;
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  param_1 = 0.0;
  pfVar2 = (float *)((int)this + 0x188);
  do {
    fVar1 = pfVar2[1] + pfVar2[2] + *pfVar2;
    pfVar2[0x19] = fVar1;
    if (param_1 < fVar1) {
      uVar5 = uVar3;
      param_1 = fVar1;
    }
    uVar3 = uVar3 + 1;
    pfVar2 = pfVar2 + 0x1d;
  } while (uVar3 < 4);
  return uVar5;
}

