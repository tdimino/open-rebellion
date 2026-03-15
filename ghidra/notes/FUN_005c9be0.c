
void __thiscall FUN_005c9be0(void *this,int param_1)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  
  iVar4 = *(int *)((int)this + 0x50);
  *(int *)((int)this + 0x38) = param_1;
  iVar2 = FUN_005aaf90();
  iVar2 = *(int *)(iVar2 + 0x9ec);
  uVar1 = *(undefined4 *)((int)this + 0x18);
  piVar3 = (int *)FUN_005aaf90();
  (**(code **)(*piVar3 + 0x28))(0x3d,uVar1,param_1,iVar4 == iVar2);
                    /* WARNING: Load size is inaccurate */
  iVar4 = (**(code **)(*this + 4))();
  if (iVar4 == 3) {
    if (param_1 == 2) {
      *(undefined4 *)((int)this + 0x5c) = 2;
      return;
    }
    *(undefined4 *)((int)this + 0x5c) = 1;
  }
  return;
}

