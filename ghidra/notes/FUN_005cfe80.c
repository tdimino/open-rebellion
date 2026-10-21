
void __thiscall FUN_005cfe80(void *this,undefined4 param_1)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  
  iVar1 = *(int *)((int)this + 8);
  *(undefined4 *)((int)this + 0x34) = 1;
  if ((iVar1 != 0) && (*(int *)(iVar1 + 0x34) == 0)) {
    uVar2 = *(undefined4 *)(iVar1 + 0x18);
    piVar3 = (int *)FUN_005aaf90();
    (**(code **)(*piVar3 + 0x28))(0x3c,uVar2,param_1,1);
  }
  return;
}

