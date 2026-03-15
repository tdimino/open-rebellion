
void __thiscall FUN_005f61c0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar3 = *(int *)((int)this + 4);
  iVar2 = 0;
  if (iVar3 == 0) {
    *(int *)((int)this + 4) = param_1;
    *(int *)((int)this + 8) = param_1;
    *(undefined4 *)(param_1 + 4) = 0;
    *(undefined4 *)(param_1 + 8) = 0;
    *(undefined4 *)(param_1 + 0xc) = 0;
    *(undefined4 *)(param_1 + 0x10) = 0;
    return;
  }
  do {
                    /* WARNING: Load size is inaccurate */
    iVar1 = (**(code **)(*this + 8))(param_1,iVar3);
    if (iVar1 == 0) {
      iVar1 = *(int *)(iVar3 + 8);
      if (iVar1 == 0) {
        *(int *)(iVar3 + 8) = param_1;
        *(int *)(param_1 + 0xc) = iVar3;
        if (iVar3 == 0) {
          *(int *)((int)this + 8) = param_1;
        }
        else {
          *(int *)(iVar3 + 0x10) = param_1;
        }
      }
    }
    else {
      iVar1 = *(int *)(iVar3 + 4);
      if (iVar1 == 0) {
        *(int *)(iVar3 + 4) = param_1;
        *(int *)(param_1 + 0xc) = iVar3;
        if (iVar2 != 0) {
          *(int *)(iVar2 + 0x10) = param_1;
          *(int *)(param_1 + 0x10) = iVar3;
          iVar3 = iVar2;
          goto LAB_005f621c;
        }
        *(int *)((int)this + 8) = param_1;
      }
      *(int *)(param_1 + 0x10) = iVar3;
      iVar3 = iVar2;
    }
LAB_005f621c:
    iVar2 = iVar3;
    iVar3 = iVar1;
    if (iVar1 == 0) {
      return;
    }
  } while( true );
}

