
bool __thiscall FUN_0054fbb0(void *this,int param_1)

{
  bool bVar1;
  bool bVar2;
  
  bVar2 = false;
  bVar1 = true;
  if (*(int *)((int)this + 0x10) == -1) {
    if (*(int *)(param_1 + 0x10) != -1) {
      bVar1 = false;
      bVar2 = true;
    }
  }
  else {
    bVar1 = false;
    bVar2 = false;
  }
  if (!bVar1) {
    return bVar2;
  }
  if (*(int *)((int)this + 0x18) == -1) {
    if (*(int *)(param_1 + 0x18) != -1) {
      bVar1 = false;
      bVar2 = true;
    }
  }
  else {
    bVar1 = false;
    bVar2 = false;
  }
  if (!bVar1) {
    return bVar2;
  }
  if (*(int *)((int)this + 4) == 0) {
    if (*(int *)(param_1 + 4) == 0) goto LAB_0054fc20;
  }
  else if (*(int *)(param_1 + 4) == 0) {
    bVar1 = false;
    bVar2 = true;
    goto LAB_0054fc20;
  }
  bVar1 = false;
  bVar2 = false;
LAB_0054fc20:
  if (bVar1) {
    bVar2 = *(int *)(param_1 + 8) < *(int *)((int)this + 8);
  }
  return bVar2;
}

