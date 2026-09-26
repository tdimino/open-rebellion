
void * __thiscall
FUN_0041acd0(void *this,int param_1,uint param_2,uint param_3,uint param_4,void *param_5)

{
  undefined4 uVar1;
  bool bVar2;
  void *this_00;
  void *pvVar3;
  undefined4 *puVar4;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b4ed;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  local_14 = (void *)FUN_00618b70(0x20);
  local_4 = 0;
  if (local_14 == (void *)0x0) {
    param_5 = (void *)0x0;
  }
  else {
    param_5 = FUN_0041be80(local_14,param_5);
  }
  local_4 = 0xffffffff;
  if (param_5 != (void *)0x0) {
    for (this_00 = (void *)thunk_FUN_005f5060((int)this + 0x78); this_00 != (void *)0x0;
        this_00 = *(void **)((int)this_00 + 0x10)) {
      if (((*(int *)((int)this_00 + 0x30) == param_1) &&
          ((param_2 & *(uint *)((int)this_00 + 0x38)) == param_2)) &&
         ((param_3 & *(uint *)((int)this_00 + 0x38)) == 0)) {
        pvVar3 = (void *)FUN_00618b70(0x18);
        local_4 = 1;
        local_10 = pvVar3;
        if (pvVar3 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          uVar1 = *(undefined4 *)((int)this_00 + (param_4 & 0xffff) * 4 + 0x50);
          puVar4 = FUN_00403040(this_00,&local_14);
          bVar2 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar3 = FUN_0041c360(pvVar3,puVar4,uVar1);
        }
        local_4 = 1;
        if (bVar2) {
          bVar2 = false;
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_0041c070(param_5,pvVar3);
      }
    }
    FUN_0041c230((int)param_5);
  }
  ExceptionList = local_c;
  return param_5;
}

