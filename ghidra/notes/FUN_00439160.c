
/* WARNING: Type propagation algorithm not settling */

void __fastcall FUN_00439160(void *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int *piVar3;
  int local_24;
  int local_20;
  int local_1c;
  int local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d7c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0060c960(*(int *)((int)param_1 + 0x114));
  if (iVar2 != 0) {
    puVar1 = *(undefined4 **)(iVar2 + 0x54);
    local_1c = 0;
    local_18[0] = 0;
    local_20 = 0;
    local_24 = 0;
    local_18[1] = 1;
    local_18[2] = 1;
    local_4 = 0;
    if ((*(int *)((int)param_1 + 0x144) != 0) && (puVar1 != (undefined4 *)0x0)) {
      FUN_00438d30((int)param_1);
      *(undefined4 *)(*(int *)((int)param_1 + 0x144) + 0x48) = *(undefined4 *)((int)param_1 + 0x148)
      ;
      FUN_004f26d0((void *)(*(int *)((int)param_1 + 0x144) + 0x44),puVar1);
      FUN_00538220(*(void **)((int)param_1 + 0x144),&local_1c,local_18,&local_20,&local_24,
                   local_18 + 1);
    }
    FUN_00438f30(param_1,(uint)(local_18[2] == -1),local_20,local_24);
    FUN_00438dd0(param_1,(uint)(local_18[2] == -1),local_18[0],local_1c);
    piVar3 = (int *)FUN_00607e90(param_1,0x65);
    if (piVar3 != (int *)0x0) {
      if (local_18[2] == -1) {
        (**(code **)(*piVar3 + 0xc))();
      }
      else {
        (**(code **)(*piVar3 + 0x10))();
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return;
}

