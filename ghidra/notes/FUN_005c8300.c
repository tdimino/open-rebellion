
void __fastcall FUN_005c8300(int *param_1)

{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  void *this;
  
  piVar2 = (int *)(**(code **)(param_1[0x1c] + 0xc))();
  while (piVar1 = piVar2, piVar1 != (int *)0x0) {
    piVar2 = (int *)(**(code **)(*piVar1 + 4))();
    piVar3 = (int *)(**(code **)(*piVar1 + 0xc))();
    iVar4 = (**(code **)(*piVar3 + 4))();
    if (iVar4 == 3) {
      iVar4 = piVar1[6];
      piVar3 = (int *)FUN_005aaf90();
      (**(code **)(*piVar3 + 0x28))(0x32,iVar4,0,1);
      iVar4 = FUN_005aaf90();
      iVar4 = *(int *)(iVar4 + 0x9ec);
      iVar5 = (**(code **)(*param_1 + 4))();
      if (iVar5 == iVar4) {
        iVar4 = piVar1[6];
        this = (void *)FUN_0059ed70();
        FUN_005a0410(this,iVar4);
      }
    }
  }
  iVar4 = (**(code **)(*param_1 + 0x20))(param_1 + 0x26,0,3);
  iVar5 = (**(code **)(*param_1 + 0x20))(param_1 + 0x28,iVar4,iVar4 + 3);
  (**(code **)(*param_1 + 0x20))(param_1 + 0x2a,iVar4 + iVar5,7);
  return;
}

