
float10 __fastcall FUN_005b18d0(int *param_1)

{
  int iVar1;
  int *piVar2;
  float10 fVar3;
  float local_4;
  
  local_4 = 0.0;
  iVar1 = 4;
  piVar2 = param_1 + 0x5f;
  do {
    if (piVar2[1] != 0) {
      fVar3 = (float10)(**(code **)(*param_1 + 0x50))();
      if ((float10)local_4 < fVar3) {
        fVar3 = (float10)(**(code **)(*param_1 + 0x50))();
        local_4 = (float)fVar3;
      }
    }
    if (*piVar2 != 0) {
      fVar3 = (float10)(**(code **)(*param_1 + 0x54))();
      if ((float10)local_4 < fVar3) {
        fVar3 = (float10)(**(code **)(*param_1 + 0x54))();
        local_4 = (float)fVar3;
      }
    }
    if (piVar2[2] != 0) {
      fVar3 = (float10)(**(code **)(*param_1 + 0x58))();
      if ((float10)local_4 < fVar3) {
        fVar3 = (float10)(**(code **)(*param_1 + 0x58))();
        local_4 = (float)fVar3;
      }
    }
    piVar2 = piVar2 + 0x1d;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  return (float10)local_4;
}

