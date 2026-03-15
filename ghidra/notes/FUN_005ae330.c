
int * __thiscall FUN_005ae330(void *this,int *param_1,int param_2,undefined4 param_3)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  void *this_00;
  int *this_01;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00652f1b;
  pvStack_c = ExceptionList;
  bVar1 = true;
  this_01 = (int *)0x0;
  ExceptionList = &pvStack_c;
  for (piVar2 = (int *)(**(code **)(*param_1 + 0x10))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
    iVar4 = (**(code **)(*piVar3 + 0x24))();
    if (iVar4 == param_2) {
      iVar4 = (**(code **)(*piVar2 + 0x10))();
      if (bVar1) {
        bVar1 = false;
        this_00 = (void *)FUN_00618b70(0xc4);
        this_01 = (int *)0x0;
        uStack_4 = 0;
        if (this_00 != (void *)0x0) {
          this_01 = (int *)FUN_005c62b0(this_00,*(uint *)(iVar4 + 0xec));
        }
        uStack_4 = 0xffffffff;
        FUN_005afe10(this_01);
        *(int *)(iVar4 + 0x174) = this_01[6];
        FUN_005c7140(this_01,param_3);
        (**(code **)(*this_01 + 8))(*(undefined4 *)(iVar4 + 0xec));
        FUN_005ca4c0((void *)(*(int *)((int)this + *(int *)(iVar4 + 0x168) * 4 + 0x24c) + 0x70),
                     (int)this_01);
      }
      else {
        (**(code **)(*this_01 + 0x28))(*(undefined4 *)(iVar4 + 0xec));
      }
    }
  }
  ExceptionList = pvStack_c;
  return this_01;
}

