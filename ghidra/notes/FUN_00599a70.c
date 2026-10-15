
void __thiscall FUN_00599a70(void *this,int param_1)

{
  undefined4 uVar1;
  void *pvVar2;
  int iVar3;
  int *piVar4;
  
  if (*(int *)((int)this + 0x668) != 0) {
    pvVar2 = (void *)FUN_00596f30();
    FUN_005ae460(pvVar2);
    *(undefined4 *)((int)this + 0x690) = 0;
    return;
  }
  FUN_00401040(param_1);
  iVar3 = FUN_005aaf90();
  uVar1 = *(undefined4 *)(iVar3 + 0x9ec);
  piVar4 = (int *)FUN_005aaf90();
  (**(code **)(*piVar4 + 0x30))(uVar1);
  *(undefined4 *)((int)this + 0x690) = 0;
  return;
}

