
void __thiscall FUN_00438b60(int *param_1,undefined4 param_2)

{
  switch(param_2) {
  case 0xd:
    FUN_00438980((int)param_1);
    (**(code **)(*param_1 + 0x30))();
    return;
  case 0x1b:
    (**(code **)(*param_1 + 0x30))();
    break;
  case 0x6b:
    FUN_00438c30(param_1);
    return;
  case 0x6d:
    FUN_00438c60(param_1);
    return;
  }
  return;
}

