
void __thiscall FUN_005ceaa0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  int iVar4;
  
  FUN_005ee3f0((int)param_1);
  iVar4 = *(int *)((int)this + 4);
  if (iVar4 == 0) {
    *(void **)((int)this + 4) = param_1;
    *(undefined4 *)((int)param_1 + 4) = 0;
    *(undefined4 *)((int)param_1 + 8) = 0;
    *(undefined4 *)((int)param_1 + 0xc) = 0;
    *(undefined4 *)((int)param_1 + 0x10) = 0;
  }
  else {
    iVar3 = 0;
    if (iVar4 != 0) {
      do {
        bVar1 = FUN_005ee370(param_1,iVar4);
        if (CONCAT31(extraout_var,bVar1) == 0) {
          iVar2 = *(int *)(iVar4 + 8);
          if (iVar2 == 0) {
            *(void **)(iVar4 + 8) = param_1;
            *(int *)((int)param_1 + 0xc) = iVar4;
            if (iVar4 != 0) {
              *(void **)(iVar4 + 0x10) = param_1;
            }
          }
        }
        else {
          iVar2 = *(int *)(iVar4 + 4);
          if (iVar2 == 0) {
            *(void **)(iVar4 + 4) = param_1;
            *(int *)((int)param_1 + 0xc) = iVar4;
            if (iVar3 != 0) {
              *(void **)(iVar3 + 0x10) = param_1;
            }
          }
          *(int *)((int)param_1 + 0x10) = iVar4;
          iVar4 = iVar3;
        }
        iVar3 = iVar4;
        iVar4 = iVar2;
      } while (iVar2 != 0);
      return;
    }
  }
  return;
}

