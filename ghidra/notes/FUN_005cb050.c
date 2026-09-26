
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * __thiscall FUN_005cb050(void *this,float *param_1,int param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  int iVar5;
  void *this_00;
  int iVar6;
  undefined4 *puVar7;
  undefined4 local_10 [4];
  
  if (*(int *)((int)this + 100) == 3) {
    iVar5 = FUN_005aaf90();
    iVar6 = *(int *)((int)this + 0x50);
    fVar4 = *(float *)(iVar5 + 0x8cc);
    puVar7 = local_10;
    this_00 = (void *)FUN_005aaf90();
    iVar6 = FUN_005a7490(this_00,puVar7,iVar6);
    fVar4 = *(float *)(iVar6 + 8) * fVar4 * _DAT_0066cc98;
    if (_DAT_0066cc9c < ABS(*(float *)(param_2 + 0x38) - fVar4)) {
      fVar1 = *(float *)(param_2 + 0x30);
      fVar2 = *(float *)(param_2 + 0x38);
      fVar3 = *(float *)(param_2 + 0x30);
      param_1[1] = *(float *)(param_2 + 0x34) - *(float *)(param_2 + 0x34);
      *param_1 = fVar1 - fVar3;
      param_1[2] = fVar4 - fVar2;
      param_1[3] = -1.0;
      return param_1;
    }
  }
  else if (*(int **)((int)this + 0x54) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x54) + 0x24))(param_1,param_2);
    return param_1;
  }
  *param_1 = DAT_006bc940;
  param_1[1] = DAT_006bc944;
  param_1[2] = DAT_006bc948;
  param_1[3] = DAT_006bc94c;
  return param_1;
}

