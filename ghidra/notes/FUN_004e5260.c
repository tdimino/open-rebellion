
void __thiscall FUN_004e5260(void *this,int *param_1)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = thunk_FUN_005f5060((int)this);
  for (bVar2 = false; (iVar1 != 0 && (!bVar2)); bVar2 = !bVar2) {
    bVar2 = *(int *)(iVar1 + 0x1c) != *param_1;
    if (bVar2) {
      iVar1 = *(int *)(iVar1 + 0x10);
    }
  }
  return;
}

