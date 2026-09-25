
void __thiscall FUN_0059c260(void *this,undefined4 *param_1,uint *param_2)

{
  void *pvVar1;
  undefined2 local_14;
  undefined2 local_12;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006519bc;
  local_c = ExceptionList;
  pvVar1 = (void *)0x0;
  local_14 = 0;
  local_12 = DAT_0065d428;
  ExceptionList = &local_c;
  local_10 = (void *)FUN_00618b70(0x114);
  local_4 = 0;
  if (local_10 != (void *)0x0) {
    pvVar1 = FUN_00602150(local_10,*param_1,0x1e5,0x110,0x1b,0x1b,this,0xad12,0x402,0x406,param_2,0,
                          0);
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(pvVar1,4,0x406);
    local_14 = 0xada2;
    FUN_005ef240(this,pvVar1,0,(int)&local_14);
  }
  pvVar1 = (void *)FUN_00618b70(0x114);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00602150(pvVar1,*param_1,0x20c,0x110,0x1b,0x1b,this,0xad13,0x403,0x407,param_2,0,0)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(pvVar1,4,0x407);
    local_14 = 0xada3;
    FUN_005ef240(this,pvVar1,0,(int)&local_14);
  }
  pvVar1 = (void *)FUN_00618b70(0x114);
  local_4 = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00602150(pvVar1,*param_1,0x233,0x110,0x1b,0x1b,this,0xad14,0x404,0x408,param_2,0,0)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(pvVar1,4,0x408);
    local_14 = 0xada4;
    FUN_005ef240(this,pvVar1,0,(int)&local_14);
  }
  pvVar1 = (void *)FUN_00618b70(0x114);
  local_4 = 3;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00602150(pvVar1,*param_1,0x25a,0x110,0x1b,0x1b,this,0xad15,0x405,0x409,param_2,0,0)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_006030c0(pvVar1,8);
    FUN_00603150(pvVar1,4,0x409);
    local_14 = 0xada5;
    FUN_005ef240(this,pvVar1,0,(int)&local_14);
  }
  ExceptionList = local_c;
  return;
}

