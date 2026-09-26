
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ec400(int param_1,float *param_2)

{
  float fVar1;
  uint uVar2;
  float fVar3;
  uint uVar4;
  longlong lVar5;
  
  uVar2 = *(uint *)(param_1 + 300);
  if (uVar2 == 1) {
    *param_2 = DAT_006bc940;
    param_2[1] = DAT_006bc944;
    param_2[2] = DAT_006bc948;
    param_2[3] = DAT_006bc94c;
    return;
  }
  uVar4 = *(uint *)(param_1 + 0x130);
  if (*(uint *)(param_1 + 0x130) <= uVar2) {
    uVar4 = uVar2;
  }
  lVar5 = __ftol();
  fVar1 = *(float *)(param_1 + 0x1c);
  fVar3 = (float)((int)lVar5 * ((int)uVar4 >> 1));
  *param_2 = -(*(float *)(param_1 + 0x24) * fVar3);
  param_2[1] = 0.0;
  param_2[2] = fVar1 * fVar3;
  param_2[3] = -1.0;
  return;
}

