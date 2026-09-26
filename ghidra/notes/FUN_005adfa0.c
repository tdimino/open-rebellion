
int * __thiscall FUN_005adfa0(void *this,int *param_1,int param_2,undefined4 param_3)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  void *this_00;
  int iVar5;
  int *this_01;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined1 *puStack_4;
  
  puStack_4 = (undefined1 *)0xffffffff;
  puStack_8 = &LAB_00652efb;
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
        puStack_4 = (undefined1 *)0x0;
        if (this_00 == (void *)0x0) {
          this_01 = (int *)0x0;
        }
        else {
          this_01 = FUN_005c62b0(this_00,*(uint *)(iVar4 + 0xec));
        }
        puStack_4 = (undefined1 *)0xffffffff;
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x48))();
        (**(code **)(*this_01 + 8))();
        puStack_4 = &stack0xffffffc4;
        FUN_005caf70(this_01,*(undefined4 *)(iVar4 + 0x30),*(undefined4 *)(iVar4 + 0x34),
                     *(undefined4 *)(iVar4 + 0x38),*(undefined4 *)(iVar4 + 0x3c));
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0x38))();
        *(int *)(iVar4 + 0x178) = this_01[6];
        *(int *)(iVar4 + 0x174) = this_01[6];
        FUN_005c7140(this_01,param_3);
        iVar5 = FUN_005aaf90();
        if (*(int *)(iVar5 + 0x9ec) == *(int *)(iVar4 + 0x168)) {
          (**(code **)(*(int *)this + 0x3c))();
        }
      }
      else {
        (**(code **)(*this_01 + 0x4c))();
        (**(code **)(*this_01 + 0x28))();
      }
    }
  }
  ExceptionList = pvStack_c;
  return this_01;
}

