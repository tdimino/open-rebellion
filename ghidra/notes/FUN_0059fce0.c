
void FUN_0059fce0(int param_1)

{
  undefined4 *puVar1;
  
  switch(*(undefined4 *)(param_1 + 0xc)) {
  case 0:
  case 2:
    puVar1 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x38))(*(undefined4 *)(param_1 + 0x10));
    FUN_0059f350(puVar1);
    break;
  case 1:
  case 3:
    puVar1 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x34))(*(undefined4 *)(param_1 + 0x10));
    FUN_0059f350(puVar1);
    return;
  }
  return;
}

