
int __fastcall FUN_00486490(void *param_1)

{
  int iVar1;
  
  iVar1 = 1;
  if (*(int **)((int)param_1 + 0xc0) != (int *)0x0) {
    iVar1 = FUN_00439950(*(int **)((int)param_1 + 0xc0));
  }
  if (iVar1 != 0) {
    FUN_00488b30((int)param_1 + 0x6c);
    FUN_00487eb0(param_1,2);
  }
  return iVar1;
}

