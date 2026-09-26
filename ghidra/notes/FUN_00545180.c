
bool __thiscall FUN_00545180(void *this,uint *param_1,undefined4 param_2,uint param_3)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  bool bVar5;
  
  bVar5 = true;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar2 = (int *)FUN_00505d40(param_1);
    bVar5 = piVar2 != (int *)0x0;
    if (piVar2 != (int *)0x0) {
      FUN_00547180(this,piVar2,param_3);
      iVar3 = thunk_FUN_00506e60();
      bVar5 = iVar3 != 0;
      if ((iVar3 != 0) && ((*(byte *)(piVar2 + 0x14) & 0x20) == 0)) {
        uVar4 = FUN_00556710(piVar2,param_3);
        if ((uVar4 != 0) && (bVar5)) {
          return true;
        }
        bVar5 = false;
      }
    }
  }
  return bVar5;
}

