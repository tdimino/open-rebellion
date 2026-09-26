
void __fastcall FUN_00438c90(int param_1)

{
  void *this;
  char *pcVar1;
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d748;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(&local_70,1);
  local_4 = 0;
  this = FUN_00616e10(local_64,*(int *)(param_1 + 0x148));
  FUN_00615f00(this,0);
  pcVar1 = (char *)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
  FUN_00604f90(*(void **)(param_1 + 0x128),pcVar1);
  FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return;
}

