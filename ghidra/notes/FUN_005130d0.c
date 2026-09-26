
int __fastcall FUN_005130d0(int param_1)

{
  undefined4 uVar1;
  int iVar2;
  bool bVar3;
  
  do {
    if (*(int *)(param_1 + 0x1c) != 0) {
      uVar1 = FUN_005c7530(*(int *)(param_1 + 0x1c));
      *(undefined4 *)(param_1 + 0x1c) = uVar1;
    }
    bVar3 = *(int *)(param_1 + 0x1c) != 0;
    if (bVar3) {
      iVar2 = FUN_005131d0(param_1);
    }
    else {
      iVar2 = 1;
    }
  } while (iVar2 == 0);
  if (!bVar3) {
    FUN_00513170(param_1);
  }
  return param_1;
}

