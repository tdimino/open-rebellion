
void __cdecl FUN_005c13d0(uint param_1,undefined4 param_2,int param_3)

{
  int *piVar1;
  void *this;
  int iVar2;
  uint uVar3;
  
  if (DAT_006bc700 != (void *)0x0) {
    if (*(int *)((int)DAT_006bc700 + 0x684) == 0) {
      if (param_3 == 2) {
        uVar3 = param_1;
        this = (void *)FUN_00596bd0();
        iVar2 = FUN_005a6340(this,uVar3);
        if (iVar2 != 0) {
          *(undefined4 *)(iVar2 + 0xf4) = 0;
          *(undefined4 *)(iVar2 + 0x100) = 0;
          *(undefined4 *)(iVar2 + 0x110) = 0;
          *(undefined4 *)(iVar2 + 0x108) = 1;
        }
        piVar1 = (int *)FUN_005c1cd0(DAT_006bc700,param_1);
        if (piVar1 != (int *)0x0) {
          (**(code **)(*piVar1 + 4))();
        }
      }
    }
    else {
      piVar1 = (int *)FUN_005c1cd0(DAT_006bc700,param_1);
      if (piVar1 != (int *)0x0) {
        (**(code **)(*piVar1 + 0x28))(param_2,param_3);
        return;
      }
    }
  }
  return;
}

