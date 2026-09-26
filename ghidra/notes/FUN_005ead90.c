
void FUN_005ead90(int param_1,float param_2)

{
  float fVar1;
  int *piVar2;
  float10 fVar3;
  float10 fVar4;
  
  piVar2 = (int *)thunk_FUN_005f5060(param_1);
  if (piVar2 != (int *)0x0) {
    fVar3 = (float10)fcos((float10)param_2);
    fVar4 = (float10)fsin((float10)param_2);
    do {
      fVar1 = (float)piVar2[0x16];
      piVar2[0x15] = piVar2[0x15];
      piVar2[0x16] = (int)((float)fVar3 * fVar1 + (float)fVar4 * (float)piVar2[0x14]);
      piVar2[0x17] = -0x40800000;
      piVar2[0x14] = (int)((float)fVar3 * (float)piVar2[0x14] - (float)fVar4 * fVar1);
      piVar2 = (int *)(**(code **)(*piVar2 + 8))();
    } while (piVar2 != (int *)0x0);
  }
  return;
}

