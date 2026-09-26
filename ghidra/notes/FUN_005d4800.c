
void __fastcall FUN_005d4800(int param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  void *pvVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065475b;
  local_c = ExceptionList;
  if (*(int *)(param_1 + 0x38) != 1) {
    ExceptionList = &local_c;
    *(undefined4 *)(param_1 + 0x38) = 1;
    if (*(int *)(param_1 + 0x4c) == 0) {
      iVar2 = (**(code **)(**(int **)(param_1 + 0x2c) + 0x48))
                        (*(int **)(param_1 + 0x2c),*(undefined4 *)(param_1 + 0x40),0);
      FUN_005d8b00(iVar2);
      uVar3 = *(undefined4 *)(param_1 + 0x40);
    }
    else {
      piVar1 = *(int **)(param_1 + 0x2c);
      if (*(int *)(param_1 + 0x4c) == 1) {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,*(undefined4 *)(param_1 + 0x44),0);
        FUN_005d8b00(iVar2);
        uVar3 = *(undefined4 *)(param_1 + 0x44);
      }
      else {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,*(undefined4 *)(param_1 + 0x48));
        FUN_005d8b00(iVar2);
        uVar3 = *(undefined4 *)(param_1 + 0x48);
      }
    }
    *(undefined4 *)(param_1 + 0x34) = uVar3;
    pvVar4 = (void *)FUN_00618b70(0x14);
    uStack_4 = 0;
    if (pvVar4 == (void *)0x0) {
      pvVar4 = (void *)0x0;
    }
    else {
      pvVar4 = FUN_005ee4f0(pvVar4,param_1);
    }
    uStack_4 = 0xffffffff;
    FUN_005f58d0((void *)(DAT_006bcbd4 + 0x5f4),(int)pvVar4);
  }
  ExceptionList = local_c;
  return;
}

