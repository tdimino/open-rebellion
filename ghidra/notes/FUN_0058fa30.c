
bool __thiscall FUN_0058fa30(void *this,int *param_1,int *param_2)

{
  int iVar1;
  int iVar2;
  char *pcVar3;
  bool bVar4;
  int local_70;
  int aiStack_6c [4];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650418;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(&local_70,1);
  local_4 = 0;
  bVar4 = false;
  if (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0) {
    FUN_00584e60(param_1,&local_70,param_2);
    iVar2 = *(int *)((int)aiStack_6c + *(int *)(local_70 + 4));
    if (*(uint *)(iVar2 + 0x1c) < *(uint *)(iVar2 + 0x18)) {
      iVar1 = 0;
    }
    else {
      iVar1 = *(uint *)(iVar2 + 0x1c) - *(uint *)(iVar2 + 0x18);
    }
    bVar4 = iVar1 != 0;
    iVar2 = FUN_00615680(iVar2);
    if ((iVar2 != 0) &&
       (pcVar3 = (char *)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4))),
       *pcVar3 == '\0')) {
      bVar4 = false;
    }
    FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
    if (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) != 0) {
      bVar4 = false;
    }
    if (bVar4) {
      FUN_00617610(&local_70,(undefined4 *)((int)this + 0x20));
    }
  }
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return bVar4;
}

