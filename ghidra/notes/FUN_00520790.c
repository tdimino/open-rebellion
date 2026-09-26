
int __thiscall FUN_00520790(void *this,void *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 local_18;
  int local_14;
  undefined4 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006435ab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f50a0(this);
  pvVar1 = param_1;
  local_14 = FUN_0051ebb0();
  if (local_14 != 0) {
    FUN_005f4d90(pvVar1,&param_1);
    iVar3 = 0;
    if (0 < (int)param_1) {
      do {
        FUN_005f4d90(pvVar1,&local_18);
        local_10 = (undefined4 *)FUN_00618b70(0x24);
        local_4 = 0;
        if (local_10 == (undefined4 *)0x0) {
          puVar2 = (undefined4 *)0x0;
        }
        else {
          puVar2 = FUN_00582a00(local_10);
        }
        local_4 = 0xffffffff;
        if (puVar2 != (undefined4 *)0x0) {
          puVar2[6] = local_18;
          FUN_005f6470(this,(int)puVar2);
        }
        iVar3 = iVar3 + 1;
      } while (iVar3 < (int)param_1);
    }
  }
  ExceptionList = local_c;
  return local_14;
}

