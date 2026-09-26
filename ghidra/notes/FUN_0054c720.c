
bool __thiscall FUN_0054c720(void *this,uint *param_1,undefined4 *param_2)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  
  bVar1 = false;
  if (*(int *)((int)this + 0x6c) != 0) {
    if ((*param_1 >> 0x18 < 0x40) || (0x7f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
  }
  bVar2 = false;
  if (bVar1) {
    bVar2 = false;
    iVar5 = 0;
    iVar4 = 0;
    do {
      if (*(int *)((int)this + 0x6c) <= iVar5) {
        return bVar2;
      }
      iVar3 = *(int *)((int)this + 0x70) + iVar4;
      if (*(uint *)(*(int *)((int)this + 0x70) + iVar4) == *param_1) {
        bVar2 = true;
        *param_2 = *(undefined4 *)(iVar3 + 4);
        param_2[1] = *(undefined4 *)(iVar3 + 8);
      }
      iVar5 = iVar5 + 1;
      iVar4 = iVar4 + 0xc;
    } while (!bVar2);
  }
  return bVar2;
}

