
int __thiscall FUN_0058e770(int param_1,uint param_2)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  int iStack_4;
  
  uVar1 = param_2;
  iStack_4 = param_1;
  iVar2 = FUN_00595510(param_2);
  if (iVar2 != 0) {
    param_2 = FUN_005f50e0();
    param_2 = param_2 & 0xffff;
    FUN_005f4db0(uVar1,&param_2);
    iStack_4 = *(int *)(param_1 + 8);
    FUN_005f4db0(uVar1,&iStack_4);
    iStack_4 = *(int *)(param_1 + 0xc);
    FUN_005f4db0(uVar1,&iStack_4);
    piVar3 = (int *)thunk_FUN_005f5060();
    do {
      if (piVar3 == (int *)0x0) {
        return iVar2;
      }
      iVar2 = (**(code **)(*piVar3 + 4))(uVar1);
      piVar3 = (int *)piVar3[4];
    } while (iVar2 != 0);
  }
  return 0;
}

