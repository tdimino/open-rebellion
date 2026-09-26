
int * __fastcall FUN_004dd8d0(void *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  
  switch(*(undefined4 *)((int)param_1 + 0x3c)) {
  case 5:
    iVar1 = FUN_004df160((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar1 != 0) & 0xfffffffb) + 0xb;
    return (int *)0x0;
  case 6:
    iVar1 = FUN_004df440((int)param_1);
    *(uint *)((int)param_1 + 0x3c) = (-(uint)(iVar1 != 0) & 0xfffffffc) + 0xb;
    return (int *)0x0;
  case 7:
    iVar1 = FUN_004df9a0((int)param_1);
    *(int *)((int)param_1 + 0x3c) = iVar1;
    if (iVar1 != 0) {
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
    *(undefined4 *)((int)param_1 + 0x20) = 1;
    return (int *)0x0;
  case 8:
    piVar3 = FUN_004dfa80((int)param_1);
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x3c) = 5;
    return (int *)0x0;
  case 0xb:
    iVar1 = FUN_004dfa70((int)param_1);
    if (iVar1 == 0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0;
      *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
      return (int *)0x0;
    }
    *(undefined4 *)((int)param_1 + 0x3c) = 8;
    return (int *)0x0;
  case 0xc:
    piVar2 = FUN_004e0a30(param_1,(uint *)((int)param_1 + 0x54));
    piVar3 = (int *)0x0;
    if (piVar2 != (int *)0x0) {
      *(undefined4 *)((int)param_1 + 0x3c) = 0xd;
      *(undefined4 *)((int)param_1 + 0x20) = 1;
      *(int *)((int)param_1 + 0x34) = *(int *)((int)param_1 + 0x34) + 1;
      return piVar2;
    }
    break;
  case 0xd:
    piVar3 = FUN_004e0b70((int)param_1);
  }
  *(undefined4 *)((int)param_1 + 0x3c) = 0;
  *(undefined4 *)((int)param_1 + 0x38) = 0x3ec;
  *(undefined4 *)((int)param_1 + 0x20) = 1;
  return piVar3;
}

