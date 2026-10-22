
int * __thiscall FUN_005ade70(void *this,int *param_1)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  
  piVar4 = (int *)0x0;
  piVar1 = (int *)(**(code **)(*param_1 + 0x10))();
  if (piVar1 != (int *)0x0) {
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    piVar4 = FUN_005aded0(this,iVar2);
    piVar1 = (int *)(**(code **)(*piVar1 + 8))();
    while (piVar1 != (int *)0x0) {
      piVar3 = (int *)(**(code **)(*piVar1 + 8))();
      piVar5 = piVar4;
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      FUN_005ade50(this,iVar2,piVar5);
      piVar1 = piVar3;
    }
  }
  return piVar4;
}

